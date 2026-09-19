// trace:STORY-1 | ai:antigravity
use crate::core::models::{PanelStatus, QueueLockState};
use dioxus::prelude::*;

#[component]
pub fn QueueLockPanel(queue_lock: PanelStatus<QueueLockState>) -> Element {
    let data = &queue_lock.data;

    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-cyan", "5" }
                    span { class: "panel-title", "Queue & Drain Lock" }
                    span { class: "panel-counter", "{data.total_queue} queued" }
                }
                div { class: "panel-meta",
                    span {
                        class: if queue_lock.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {queue_lock.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                div { style: "display: flex; gap: 12px; margin-bottom: 16px; flex-wrap: wrap;",
                    div { class: "metric-card", style: "flex: 1; min-width: 160px;",
                        span { class: "metric-label", "Drain Lock" }
                        div { class: "metric-value", style: if data.lock_held { "color: var(--color-rose);" } else { "color: var(--color-emerald);" },
                            if data.lock_held {
                                if let Some(pid) = data.locked_by_pid {
                                    "HELD (PID {pid})"
                                } else {
                                    "HELD"
                                }
                            } else {
                                "FREE / IDLE"
                            }
                        }
                        span { class: "metric-sub mono",
                            if let Some(wave) = &data.active_wave {
                                "Wave: {wave}"
                            } else {
                                "No active wave"
                            }
                        }
                    }

                    for (role, count) in &data.role_depths {
                        div { class: "metric-card", style: "flex: 1; min-width: 120px;",
                            span { class: "metric-label", "Role: {role}" }
                            div { class: "metric-value mono", "{count}" }
                            span { class: "metric-sub", "Active sessions" }
                        }
                    }
                }

                if !data.active_sessions.is_empty() {
                    div { style: "font-size: 12px; font-weight: 700; margin-bottom: 8px; color: var(--text-secondary);",
                        "Live Fleet Sessions"
                    }
                    table { class: "data-table",
                        thead {
                            tr {
                                th { "Session ID" }
                                th { "Role" }
                                th { "Spec" }
                                th { "PID" }
                                th { "Elapsed" }
                                th { "Liveness" }
                            }
                        }
                        tbody {
                            for s in &data.active_sessions {
                                tr {
                                    td { class: "mono", "{s.session_id}" }
                                    td { span { class: "badge badge-phase", "{s.role}" } }
                                    td {
                                        if let Some(spec) = &s.spec {
                                            span { class: "badge badge-spec mono", "{spec}" }
                                        } else {
                                            span { style: "color: var(--text-muted);", "-" }
                                        }
                                    }
                                    td { class: "mono",
                                        if let Some(pid) = s.pid {
                                            "{pid}"
                                        } else {
                                            "-"
                                        }
                                    }
                                    td { class: "mono", "{s.elapsed_secs / 60}m {s.elapsed_secs % 60}s" }
                                    td {
                                        if s.live {
                                            span { class: "badge badge-green", "Live" }
                                        } else {
                                            span { class: "badge badge-round", "{s.liveness}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "panel-footer",
                span { "Role queue depths and process session table" }
                span { class: "mono", "Source: {queue_lock.source_command}" }
            }
        }
    }
}
