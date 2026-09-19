// trace:STORY-1 | ai:antigravity
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventItem {
    pub ts: DateTime<Utc>,
    #[serde(default)]
    pub spec: Option<String>,
    #[serde(default)]
    pub run_uuid: Option<String>,
    pub kind: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundInFlight {
    pub spec: String,
    pub round: u32,
    pub shelved_count: u32,
    pub current_phase: Option<String>,
    pub round_started_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub elapsed_round_secs: u64,
    pub last_shelve_cause: Option<String>,
    pub last_shelve_detail: Option<String>,
    pub last_shelve_recovery_hint: Option<String>,
    pub run_uuid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseClockItem {
    pub spec: Option<String>,
    pub phase: String,
    pub seat: Option<String>,
    pub entered_at: DateTime<Utc>,
    pub elapsed_secs: u64,
    pub median_secs: u64,
    pub is_over_median: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateHeldPr {
    pub pr_number: u64,
    pub title: String,
    pub held_at: Option<DateTime<Utc>>,
    pub age_secs: u64,
    pub verdict: Option<String>,
    pub needs_attention: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShelveCauses24h {
    pub total_shelves: usize,
    pub counts_by_cause: BTreeMap<String, usize>,
    pub percentages: BTreeMap<String, f64>,
    pub primary_diagnostic: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionInfo {
    pub session_id: String,
    pub spec: Option<String>,
    pub role: String,
    pub pid: Option<u32>,
    pub elapsed_secs: u64,
    pub live: bool,
    pub liveness: String,
    pub dispatch_state: Option<String>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct QueueLockState {
    pub lock_held: bool,
    pub locked_by_pid: Option<u32>,
    pub locked_duration_secs: Option<u64>,
    pub role_depths: BTreeMap<String, usize>,
    pub total_queue: usize,
    pub active_sessions: Vec<SessionInfo>,
    pub active_wave: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CiStatusItem {
    pub run_id: Option<String>,
    pub conclusion: String,
    pub duration_secs: Option<u64>,
    pub spec: Option<String>,
    pub pr: Option<u64>,
    pub ts: DateTime<Utc>,
    pub is_green: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SeatResponsiveness {
    pub unread_mail: BTreeMap<String, usize>,
    pub oldest_mail_age_secs: Option<u64>,
    pub active_jobs: usize,
    pub due_jobs: usize,
    pub overdue_jobs: usize,
    pub next_job_run: Option<String>,
    pub findings_total: usize,
    pub pending_briefs: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ThroughputStats {
    pub last_merge: Option<DateTime<Utc>>,
    pub minutes_since_last_merge: Option<u64>,
    pub merges_last_day: usize,
    pub merges_last_hour: usize,
    pub main_idle: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelStatus<T> {
    pub data: T,
    pub last_updated: DateTime<Utc>,
    pub is_stale: bool,
    pub source_command: String,
    pub error_message: Option<String>,
}

impl<T: Default> Default for PanelStatus<T> {
    fn default() -> Self {
        Self {
            data: T::default(),
            last_updated: Utc::now(),
            is_stale: true,
            source_command: String::new(),
            error_message: None,
        }
    }
}

impl<T> PanelStatus<T> {
    pub fn ok(data: T, source: &str) -> Self {
        Self {
            data,
            last_updated: Utc::now(),
            is_stale: false,
            source_command: source.to_string(),
            error_message: None,
        }
    }

    pub fn stale(data: T, source: &str, error: Option<String>) -> Self {
        Self {
            data,
            last_updated: Utc::now(),
            is_stale: true,
            source_command: source.to_string(),
            error_message: error,
        }
    }

    pub fn age_display(&self) -> String {
        let secs = (Utc::now() - self.last_updated).num_seconds().max(0);
        if secs < 60 {
            format!("{}s ago", secs)
        } else if secs < 3600 {
            format!("{}m ago", secs / 60)
        } else {
            format!("{}h ago", secs / 3600)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardSnapshot {
    pub project_path: String,
    pub project_name: String,
    pub timestamp: DateTime<Utc>,
    pub feed_event_count: usize,
    pub rounds: PanelStatus<Vec<RoundInFlight>>,
    pub phases: PanelStatus<Vec<PhaseClockItem>>,
    pub gate: PanelStatus<Vec<GateHeldPr>>,
    pub shelve_causes: PanelStatus<ShelveCauses24h>,
    pub queue_lock: PanelStatus<QueueLockState>,
    pub ci: PanelStatus<Vec<CiStatusItem>>,
    pub seats: PanelStatus<SeatResponsiveness>,
    pub throughput: PanelStatus<ThroughputStats>,
    pub recent_events: Vec<EventItem>,
}
