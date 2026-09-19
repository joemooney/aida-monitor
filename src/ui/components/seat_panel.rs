// trace:STORY-1 | ai:antigravity
use crate::core::models::{PanelStatus, SeatResponsiveness};
use dioxus::prelude::*;

#[component]
pub fn SeatPanel(seats: PanelStatus<SeatResponsiveness>) -> Element {
    let data = &seats.data;

    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-amber", "7" }
                    span { class: "panel-title", "Seat Responsiveness & Schedule" }
                }
                div { class: "panel-meta",
                    span {
                        class: if seats.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {seats.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 10px; margin-bottom: 14px;",
                    div { class: "metric-card",
                        span { class: "metric-label", "Findings Triage" }
                        div { class: "metric-value mono", style: if data.findings_total > 0 { "color: var(--color-amber);" } else { "color: var(--text-primary);" },
                            "{data.findings_total}"
                        }
                        span { class: "metric-sub", "Awaiting human review" }
                    }

                    div { class: "metric-card",
                        span { class: "metric-label", "Scheduled Jobs" }
                        div { class: "metric-value mono", "{data.active_jobs}" }
                        span { class: "metric-sub", "Active cron jobs" }
                    }

                    div { class: "metric-card",
                        span { class: "metric-label", "Overdue Jobs" }
                        div { class: "metric-value mono", style: if data.overdue_jobs > 0 { "color: var(--color-rose);" } else { "color: var(--color-emerald);" },
                            "{data.overdue_jobs}"
                        }
                        span { class: "metric-sub",
                            if data.overdue_jobs > 0 { "Needs attention" } else { "All jobs on time" }
                        }
                    }

                    div { class: "metric-card",
                        span { class: "metric-label", "Pending Briefs" }
                        div { class: "metric-value mono", "{data.pending_briefs}" }
                        span { class: "metric-sub", "Agent briefings queue" }
                    }
                }

                if !data.unread_mail.is_empty() {
                    div { style: "margin-top: 12px;",
                        div { style: "font-size: 12px; font-weight: 700; color: var(--text-secondary); margin-bottom: 6px;", "Unread Mail by Seat" }
                        div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                            for (seat, count) in &data.unread_mail {
                                div { class: "badge badge-round mono", style: "padding: 4px 8px;",
                                    "{seat}: {count} unread"
                                }
                            }
                        }
                    }
                }
            }

            div { class: "panel-footer",
                span { "Derived from aida awaiting and aida schedule status" }
                span { class: "mono", "Source: {seats.source_command}" }
            }
        }
    }
}
