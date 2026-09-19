// trace:STORY-1 | ai:antigravity
// trace:TASK-2 | ai:antigravity
use super::feed::EventFeedState;
use super::models::{
    DashboardSnapshot, GateHeldPr, PanelStatus, QueueLockState, SeatResponsiveness, SessionInfo,
    ThroughputStats,
};
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct Collector {
    pub project_root: PathBuf,
    pub feed_state: EventFeedState,
    pub last_snapshot: DashboardSnapshot,
}

impl Collector {
    pub fn new<P: AsRef<Path>>(project_root: P) -> Self {
        let root = project_root.as_ref().to_path_buf();
        let feed_state = EventFeedState::new(&root);
        let project_name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let snapshot = DashboardSnapshot {
            project_path: root.display().to_string(),
            project_name,
            timestamp: Utc::now(),
            ..Default::default()
        };

        Self {
            project_root: root,
            feed_state,
            last_snapshot: snapshot,
        }
    }

    /// Refresh event feed and derived panels (Rounds in flight, Phase clock, Shelve causes)
    pub fn refresh_feed(&mut self) -> Result<bool, String> {
        let changed = self.feed_state.poll_new_events()?;
        self.last_snapshot.feed_event_count = self.feed_state.all_events.len();

        // Always update derived feed panels so elapsed clocks advance smoothly
        let rounds = self.feed_state.derive_rounds_in_flight();
        self.last_snapshot.rounds = PanelStatus::ok(rounds, "tail .aida/events.jsonl");

        let phases = self.feed_state.derive_phase_clock();
        self.last_snapshot.phases = PanelStatus::ok(phases, "tail .aida/events.jsonl");

        let shelve_causes = self.feed_state.derive_shelve_causes_24h();
        self.last_snapshot.shelve_causes =
            PanelStatus::ok(shelve_causes, "tail .aida/events.jsonl");

        let feed_ci = self.feed_state.derive_recent_ci();
        if !feed_ci.is_empty() && self.last_snapshot.ci.data.is_empty() {
            self.last_snapshot.ci = PanelStatus::ok(feed_ci, "tail .aida/events.jsonl");
        }

        self.last_snapshot.recent_events = self
            .feed_state
            .all_events
            .iter()
            .rev()
            .take(40)
            .cloned()
            .collect();

        self.last_snapshot.timestamp = Utc::now();
        Ok(changed)
    }

    /// Run an AIDA CLI command safely and parse JSON with strict timeout safeguard
    fn run_command_json(&self, args: &[&str]) -> Result<serde_json::Value, String> {
        let mut cmd = Command::new("aida");
        cmd.current_dir(&self.project_root);
        // Guard against any GitHub / forge API calls or remote syncs on hot paths
        cmd.env("AIDA_OFFLINE", "1");
        cmd.env("AIDA_NO_SYNC", "1");
        cmd.env("GIT_TERMINAL_PROMPT", "0");
        cmd.env("AIDA_OUTPUT_FORMAT", "json");
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        cmd.args(args);

        let child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn aida: {}", e))?;

        let pid = child.id();
        let timeout = std::time::Duration::from_millis(3500);
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let res = child.wait_with_output();
            let _ = tx.send(res);
        });

        match rx.recv_timeout(timeout) {
            Ok(Ok(output)) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!(
                        "Command exited with {}: {}",
                        output.status,
                        stderr.trim()
                    ));
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<serde_json::Value>(stdout.trim())
                    .map_err(|e| format!("Invalid JSON response: {}", e))
            }
            Ok(Err(e)) => Err(format!("Failed to wait on command: {}", e)),
            Err(_) => {
                // Terminate runaway child process
                let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
                Err(format!(
                    "Command timed out after {}ms (degraded to protect dashboard performance)",
                    timeout.as_millis()
                ))
            }
        }
    }

    /// Poll `aida ps --json` & `aida drain status --json`
    pub fn refresh_queue_and_lock(&mut self) {
        let ps_res = self.run_command_json(&["ps", "--json"]);
        let drain_res = self.run_command_json(&["drain", "status", "--json"]);

        match ps_res {
            Ok(ps_val) => {
                let mut state = QueueLockState::default();
                let mut active_sessions = Vec::new();

                if let Some(sessions_arr) = ps_val.get("sessions").and_then(|s| s.as_array()) {
                    for s in sessions_arr {
                        let session_id = s
                            .get("session_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let spec = s
                            .get("spec")
                            .and_then(|v| v.as_str())
                            .map(|v| v.to_string());
                        let role = s
                            .get("role")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let pid = s.get("pid").and_then(|v| v.as_u64()).map(|v| v as u32);
                        let elapsed_secs =
                            s.get("elapsed_secs").and_then(|v| v.as_u64()).unwrap_or(0);
                        let live = s.get("live").and_then(|v| v.as_bool()).unwrap_or(false);
                        let liveness = s
                            .get("liveness")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let dispatch_state = s
                            .get("dispatch_state")
                            .and_then(|v| v.as_str())
                            .map(|v| v.to_string());
                        let branch = s
                            .get("branch")
                            .and_then(|v| v.as_str())
                            .map(|v| v.to_string());

                        if live {
                            *state.role_depths.entry(role.clone()).or_insert(0) += 1;
                        }

                        active_sessions.push(SessionInfo {
                            session_id,
                            spec,
                            role,
                            pid,
                            elapsed_secs,
                            live,
                            liveness,
                            dispatch_state,
                            branch,
                        });
                    }
                }

                if let Ok(drain_val) = drain_res {
                    let status = drain_val
                        .get("status")
                        .and_then(|v| v.as_str())
                        .unwrap_or("none");
                    if status != "none" {
                        state.active_wave = Some(status.to_string());
                        state.lock_held = true;
                    }
                }

                state.total_queue = state.role_depths.values().sum();
                state.active_sessions = active_sessions;

                self.last_snapshot.queue_lock = PanelStatus::ok(state, "aida ps --json");
            }
            Err(e) => {
                let prev_data = self.last_snapshot.queue_lock.data.clone();
                self.last_snapshot.queue_lock =
                    PanelStatus::stale(prev_data, "aida ps --json", Some(e));
            }
        }
    }

    /// Poll `aida merge-hold list --json` (At the gate)
    pub fn refresh_merge_holds(&mut self) {
        match self.run_command_json(&["merge-hold", "list", "--json"]) {
            Ok(val) => {
                let mut held_prs = Vec::new();
                if let Some(arr) = val.get("merge_holds").and_then(|a| a.as_array()) {
                    for item in arr {
                        let pr_number = item.get("pr").and_then(|v| v.as_u64()).unwrap_or(0);
                        let title = item
                            .get("title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let verdict = item
                            .get("verdict")
                            .and_then(|v| v.as_str())
                            .map(|v| v.to_string());
                        let age_secs = item.get("age_secs").and_then(|v| v.as_u64()).unwrap_or(0);
                        let held_at = item
                            .get("held_at")
                            .and_then(|v| v.as_str())
                            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                            .map(|d| d.with_timezone(&Utc));

                        held_prs.push(GateHeldPr {
                            pr_number,
                            title,
                            held_at,
                            age_secs,
                            verdict,
                            needs_attention: age_secs > 7200,
                        });
                    }
                }
                self.last_snapshot.gate = PanelStatus::ok(held_prs, "aida merge-hold list --json");
            }
            Err(e) => {
                let prev = self.last_snapshot.gate.data.clone();
                self.last_snapshot.gate =
                    PanelStatus::stale(prev, "aida merge-hold list --json", Some(e));
            }
        }
    }

    /// Poll `aida awaiting --json` & `aida schedule status --json` (Seat responsiveness)
    pub fn refresh_seats(&mut self) {
        let awaiting_res = self.run_command_json(&["awaiting", "--json"]);
        let schedule_res = self.run_command_json(&["schedule", "status", "--json"]);

        let mut seats = SeatResponsiveness::default();

        if let Ok(val) = awaiting_res {
            seats.findings_total = val
                .get("findings_total")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;
            if let Some(mail_obj) = val.get("mail").and_then(|m| m.as_object()) {
                let unread = mail_obj.get("unread").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                if unread > 0 {
                    seats.unread_mail.insert("current".to_string(), unread);
                }
                let shared_unread = mail_obj
                    .get("shared_unread")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize;
                if shared_unread > 0 {
                    let scope = mail_obj
                        .get("shared_scope")
                        .and_then(|s| s.as_str())
                        .unwrap_or("shared");
                    seats.unread_mail.insert(scope.to_string(), shared_unread);
                }
            }
            if let Some(briefs) = val.get("pending_briefs").and_then(|b| b.as_array()) {
                seats.pending_briefs = briefs.len();
            }
        }

        if let Ok(val) = schedule_res {
            seats.active_jobs =
                val.get("active_jobs").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            seats.due_jobs = val.get("due").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            seats.overdue_jobs = val
                .get("overdue_jobs")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;
            seats.next_job_run = val
                .get("next")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string());
        }

        self.last_snapshot.seats = PanelStatus::ok(seats, "aida awaiting --json");
    }

    /// Poll `aida integrate --json` (Throughput & CI trend)
    pub fn refresh_integrate(&mut self) {
        match self.run_command_json(&["integrate", "--json"]) {
            Ok(val) => {
                let mut stats = ThroughputStats::default();
                if let Some(tp) = val.get("throughput") {
                    stats.last_merge = tp
                        .get("last_merge")
                        .and_then(|v| v.as_str())
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                        .map(|d| d.with_timezone(&Utc));
                    stats.minutes_since_last_merge =
                        tp.get("minutes_since_last_merge").and_then(|v| v.as_u64());
                    stats.merges_last_day = tp
                        .get("merges_last_day")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;
                    stats.merges_last_hour = tp
                        .get("merges_last_hour")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;
                    stats.main_idle = tp
                        .get("main_idle")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);
                }
                self.last_snapshot.throughput = PanelStatus::ok(stats, "aida integrate --json");
            }
            Err(e) => {
                let prev = self.last_snapshot.throughput.data.clone();
                self.last_snapshot.throughput =
                    PanelStatus::stale(prev, "aida integrate --json", Some(e));
            }
        }
    }

    /// Gather full snapshot synchronously
    pub fn gather_full_snapshot(&mut self) -> &DashboardSnapshot {
        let _ = self.refresh_feed();
        self.refresh_queue_and_lock();
        self.refresh_merge_holds();
        self.refresh_seats();
        self.refresh_integrate();
        &self.last_snapshot
    }
}
