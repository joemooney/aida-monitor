// trace:STORY-1 | ai:antigravity
use super::models::{CiStatusItem, EventItem, PhaseClockItem, RoundInFlight, ShelveCauses24h};
use chrono::{DateTime, Duration, Utc};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct EventFeedState {
    pub file_path: PathBuf,
    pub last_offset: u64,
    pub all_events: Vec<EventItem>,
}

impl EventFeedState {
    pub fn new<P: AsRef<Path>>(project_root: P) -> Self {
        let file_path = project_root.as_ref().join(".aida").join("events.jsonl");
        Self {
            file_path,
            last_offset: 0,
            all_events: Vec::new(),
        }
    }

    /// Incremental poll of the event feed. Returns true if new events were read.
    pub fn poll_new_events(&mut self) -> Result<bool, String> {
        if !self.file_path.exists() {
            return Ok(false);
        }

        let file = File::open(&self.file_path)
            .map_err(|e| format!("Failed to open {}: {}", self.file_path.display(), e))?;
        let mut reader = BufReader::new(file);

        let metadata = self.file_path.metadata().map_err(|e| e.to_string())?;
        let len = metadata.len();

        // Handle file truncation or rotation
        if len < self.last_offset {
            self.last_offset = 0;
            self.all_events.clear();
        }

        reader
            .seek(SeekFrom::Start(self.last_offset))
            .map_err(|e| e.to_string())?;

        let mut line = String::new();
        let mut new_count = 0;

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                self.last_offset += bytes_read as u64;
                continue;
            }

            // Parse JSON line. Degrade gracefully on corrupt lines
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                if let Some(ts_str) = val.get("ts").and_then(|t| t.as_str()) {
                    if let Ok(ts) = DateTime::parse_from_rfc3339(ts_str) {
                        let spec = val
                            .get("spec")
                            .and_then(|s| s.as_str())
                            .map(|s| s.to_string());
                        let run_uuid = val
                            .get("run_uuid")
                            .and_then(|u| u.as_str())
                            .map(|u| u.to_string());
                        let kind = val.get("kind").cloned().unwrap_or(serde_json::Value::Null);

                        self.all_events.push(EventItem {
                            ts: ts.with_timezone(&Utc),
                            spec,
                            run_uuid,
                            kind,
                        });
                        new_count += 1;
                    }
                }
            }

            self.last_offset += bytes_read as u64;
        }

        Ok(new_count > 0)
    }

    /// Derive Rounds in Flight
    pub fn derive_rounds_in_flight(&self) -> Vec<RoundInFlight> {
        let now = Utc::now();
        // Track per spec:
        #[derive(Default)]
        struct SpecState {
            shelved_count: u32,
            round: u32,
            current_phase: Option<String>,
            round_started_at: Option<DateTime<Utc>>,
            last_activity: Option<DateTime<Utc>>,
            last_cause: Option<String>,
            last_detail: Option<String>,
            last_hint: Option<String>,
            last_run_uuid: Option<String>,
            is_completed: bool,
            is_shelved: bool,
        }

        let mut map: HashMap<String, SpecState> = HashMap::new();

        for ev in &self.all_events {
            let Some(spec) = &ev.spec else { continue };
            let entry = map.entry(spec.clone()).or_default();
            entry.last_activity = Some(ev.ts);

            if let Some(uuid) = &ev.run_uuid {
                entry.last_run_uuid = Some(uuid.clone());
            }

            let event_name = ev.kind.get("event").and_then(|e| e.as_str()).unwrap_or("");

            match event_name {
                "RunStarted" => {
                    entry.is_completed = false;
                    entry.is_shelved = false;
                    if entry.round == 0 {
                        entry.round = 1;
                    }
                    if entry.round_started_at.is_none() {
                        entry.round_started_at = Some(ev.ts);
                    }
                }
                "PhaseEntered" => {
                    entry.is_completed = false;
                    entry.is_shelved = false;
                    if entry.round == 0 {
                        entry.round = 1;
                    }
                    if entry.round_started_at.is_none() {
                        entry.round_started_at = Some(ev.ts);
                    }
                    let slug = ev
                        .kind
                        .get("slug")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string());
                    entry.current_phase = slug;
                }
                "SpecShelved" => {
                    entry.shelved_count += 1;
                    entry.is_shelved = true;
                    entry.last_cause = ev
                        .kind
                        .get("kind")
                        .and_then(|k| k.as_str())
                        .map(|k| k.to_string());
                    entry.last_detail = ev
                        .kind
                        .get("detail")
                        .and_then(|d| d.as_str())
                        .map(|d| d.to_string());
                    entry.last_hint = ev
                        .kind
                        .get("recovery_hint")
                        .and_then(|h| h.as_str())
                        .map(|h| h.to_string());
                }
                "SpecRetried" => {
                    entry.round += 1;
                    entry.is_shelved = false;
                    entry.round_started_at = Some(ev.ts);
                    if let Some(cause) = ev.kind.get("cause").and_then(|c| c.as_str()) {
                        entry.last_cause = Some(cause.to_string());
                    }
                }
                "PrMerged" | "PhaseDonePr" => {
                    entry.is_completed = true;
                    entry.is_shelved = false;
                }
                _ => {}
            }
        }

        let mut results = Vec::new();
        for (spec, state) in map {
            // Keep specs that had activity in the last 72 hours and are not completed
            let last_active = state.last_activity.unwrap_or(now);
            let age = (now - last_active).num_hours();
            if age <= 72 && !state.is_completed {
                let round_start = state.round_started_at.unwrap_or(last_active);
                let elapsed_secs = (now - round_start).num_seconds().max(0) as u64;

                results.push(RoundInFlight {
                    spec,
                    round: state.round.max(1),
                    shelved_count: state.shelved_count,
                    current_phase: if state.is_shelved {
                        Some("shelved".to_string())
                    } else {
                        state.current_phase
                    },
                    round_started_at: round_start,
                    last_activity_at: last_active,
                    elapsed_round_secs: elapsed_secs,
                    last_shelve_cause: state.last_cause,
                    last_shelve_detail: state.last_detail,
                    last_shelve_recovery_hint: state.last_hint,
                    run_uuid: state.last_run_uuid,
                });
            }
        }

        // Sort: active/shelved rounds descending by round or shelved count
        results.sort_by(|a, b| {
            b.shelved_count
                .cmp(&a.shelved_count)
                .then(b.round.cmp(&a.round))
                .then(a.spec.cmp(&b.spec))
        });

        results
    }

    /// Derive Phase Clock items for active phases
    pub fn derive_phase_clock(&self) -> Vec<PhaseClockItem> {
        let now = Utc::now();
        let mut active_phases: HashMap<String, PhaseClockItem> = HashMap::new();

        for ev in &self.all_events {
            let Some(spec) = &ev.spec else { continue };
            let event_name = ev.kind.get("event").and_then(|e| e.as_str()).unwrap_or("");

            match event_name {
                "PhaseEntered" => {
                    let phase = ev
                        .kind
                        .get("slug")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let seat = ev
                        .kind
                        .get("seat")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string());
                    let median_secs = match phase.as_str() {
                        "ci" => 900,
                        "implementer" => 1200,
                        "reviewer" => 600,
                        "merge" | "pull" | "build" => 180,
                        _ => 600,
                    };
                    let elapsed_secs = (now - ev.ts).num_seconds().max(0) as u64;
                    active_phases.insert(
                        spec.clone(),
                        PhaseClockItem {
                            spec: Some(spec.clone()),
                            phase,
                            seat,
                            entered_at: ev.ts,
                            elapsed_secs,
                            median_secs,
                            is_over_median: elapsed_secs > median_secs,
                        },
                    );
                }
                "SpecShelved" | "PrMerged" | "PhaseDonePr" | "QueueDrained" => {
                    active_phases.remove(spec);
                }
                _ => {}
            }
        }

        let mut items: Vec<PhaseClockItem> = active_phases
            .into_values()
            .filter(|p| (now - p.entered_at).num_hours() <= 24)
            .collect();

        // Sort by elapsed seconds descending (longest sitting phases first)
        items.sort_by(|a, b| b.elapsed_secs.cmp(&a.elapsed_secs));
        items
    }

    /// Derive Shelve Causes in the rolling 24-hour window
    pub fn derive_shelve_causes_24h(&self) -> ShelveCauses24h {
        let now = Utc::now();
        let cutoff = now - Duration::hours(24);

        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut total = 0;

        for ev in &self.all_events {
            if ev.ts < cutoff {
                continue;
            }
            let event_name = ev.kind.get("event").and_then(|e| e.as_str()).unwrap_or("");

            if event_name == "SpecShelved" {
                total += 1;
                let raw_kind = ev
                    .kind
                    .get("kind")
                    .and_then(|k| k.as_str())
                    .unwrap_or("unknown");

                let normalized = if raw_kind.contains("ci") || raw_kind.contains("red") {
                    if raw_kind.contains("unavailable") {
                        "ci-unavailable"
                    } else {
                        "ci-red"
                    }
                } else if raw_kind.contains("review") || raw_kind.contains("verdict") {
                    "reviewer-changes"
                } else if raw_kind.contains("tool") || raw_kind.contains("exit") {
                    "tool-exit"
                } else if raw_kind.contains("timeout") || raw_kind.contains("idle") {
                    "timeout"
                } else {
                    raw_kind
                };

                *counts.entry(normalized.to_string()).or_insert(0) += 1;
            }
        }

        let mut percentages = BTreeMap::new();
        for (cause, count) in &counts {
            let pct = if total > 0 {
                (*count as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            percentages.insert(cause.clone(), (pct * 10.0).round() / 10.0);
        }

        // Diagnostic summary
        let primary_diagnostic = if total == 0 {
            "No shelves in rolling 24h. Pipeline running clean.".to_string()
        } else {
            let max_cause = counts.iter().max_by_key(|(_, &c)| c);
            match max_cause.map(|(k, _)| k.as_str()) {
                Some("reviewer-changes") => {
                    "Diagnostic: Majority reviewer change-requests indicates specs may be under-specified or require clearer acceptance criteria.".to_string()
                }
                Some("ci-red") => {
                    "Diagnostic: Majority red CI indicates local pre-merge verification guards are running too late or tests are flaky.".to_string()
                }
                Some("ci-unavailable") => {
                    "Diagnostic: Majority CI-unavailable indicates forge rate-limiting or network connectivity bottlenecks.".to_string()
                }
                Some("tool-exit") => {
                    "Diagnostic: Majority tool-exit indicates environment instability or coding agent subprocess crash.".to_string()
                }
                _ => format!("Diagnostic: {} shelves recorded across multiple causes.", total),
            }
        };

        ShelveCauses24h {
            total_shelves: total,
            counts_by_cause: counts,
            percentages,
            primary_diagnostic,
        }
    }

    /// Derive Recent CI conclusions from feed events
    pub fn derive_recent_ci(&self) -> Vec<CiStatusItem> {
        let mut list = Vec::new();

        for ev in self.all_events.iter().rev() {
            let event_name = ev.kind.get("event").and_then(|e| e.as_str()).unwrap_or("");
            if event_name == "CiTerminal" {
                let green = ev
                    .kind
                    .get("green")
                    .and_then(|g| g.as_bool())
                    .unwrap_or(false);
                list.push(CiStatusItem {
                    run_id: ev.run_uuid.clone(),
                    conclusion: if green {
                        "success".to_string()
                    } else {
                        "failure".to_string()
                    },
                    duration_secs: None,
                    spec: ev.spec.clone(),
                    pr: None,
                    ts: ev.ts,
                    is_green: green,
                });
            }
            if list.len() >= 20 {
                break;
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_derives_rounds_and_shelves() {
        let mut feed = EventFeedState::default();
        let now = Utc::now();

        // 1. Spec starts
        feed.all_events.push(EventItem {
            ts: now - Duration::minutes(45),
            spec: Some("TASK-100".to_string()),
            run_uuid: Some("uuid-1".to_string()),
            kind: serde_json::json!({"event": "RunStarted"}),
        });

        // 2. Enters implementer
        feed.all_events.push(EventItem {
            ts: now - Duration::minutes(40),
            spec: Some("TASK-100".to_string()),
            run_uuid: Some("uuid-1".to_string()),
            kind: serde_json::json!({"event": "PhaseEntered", "slug": "implementer"}),
        });

        // 3. Shelved due to ci-red
        feed.all_events.push(EventItem {
            ts: now - Duration::minutes(25),
            spec: Some("TASK-100".to_string()),
            run_uuid: Some("uuid-1".to_string()),
            kind: serde_json::json!({
                "event": "SpecShelved",
                "kind": "ci-red",
                "detail": "CI failed on unit tests",
                "recovery_hint": "Fix test assertion"
            }),
        });

        // 4. Retried into round 2
        feed.all_events.push(EventItem {
            ts: now - Duration::minutes(15),
            spec: Some("TASK-100".to_string()),
            run_uuid: Some("uuid-1".to_string()),
            kind: serde_json::json!({"event": "SpecRetried", "cause": "ci-red"}),
        });

        // 5. Enters CI phase
        feed.all_events.push(EventItem {
            ts: now - Duration::minutes(10),
            spec: Some("TASK-100".to_string()),
            run_uuid: Some("uuid-1".to_string()),
            kind: serde_json::json!({"event": "PhaseEntered", "slug": "ci"}),
        });

        // Unknown future event should be ignored without panic
        feed.all_events.push(EventItem {
            ts: now - Duration::minutes(5),
            spec: Some("TASK-100".to_string()),
            run_uuid: Some("uuid-1".to_string()),
            kind: serde_json::json!({"event": "FutureQuantumPhase", "extra": 42}),
        });

        let rounds = feed.derive_rounds_in_flight();
        assert_eq!(rounds.len(), 1);
        let r = &rounds[0];
        assert_eq!(r.spec, "TASK-100");
        assert_eq!(r.round, 2);
        assert_eq!(r.shelved_count, 1);
        assert_eq!(r.last_shelve_cause.as_deref(), Some("ci-red"));
        assert_eq!(
            r.last_shelve_detail.as_deref(),
            Some("CI failed on unit tests")
        );
        assert_eq!(r.current_phase.as_deref(), Some("ci"));

        let phases = feed.derive_phase_clock();
        assert_eq!(phases.len(), 1);
        assert_eq!(phases[0].phase, "ci");

        let causes = feed.derive_shelve_causes_24h();
        assert_eq!(causes.total_shelves, 1);
        assert_eq!(causes.counts_by_cause.get("ci-red"), Some(&1));
    }
}
