// trace:STORY-1 | ai:antigravity
use crate::core::models::{CiStatusItem, PanelStatus, ThroughputStats};
use dioxus::prelude::*;

#[component]
pub fn CiPanel(
    ci: PanelStatus<Vec<CiStatusItem>>,
    throughput: PanelStatus<ThroughputStats>,
) -> Element {
    let tp = &throughput.data;

    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-emerald", "6" }
                    span { class: "panel-title", "CI & Integration Throughput" }
                }
                div { class: "panel-meta",
                    span {
                        class: if ci.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {ci.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                // Throughput Metric Cards
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(130px, 1fr)); gap: 10px; margin-bottom: 16px;",
                    div { class: "metric-card",
                        span { class: "metric-label", "Merges (24h)" }
                        div { class: "metric-value mono", style: "color: var(--color-emerald);", "{tp.merges_last_day}" }
                        span { class: "metric-sub", "Past 24 hours" }
                    }
                    div { class: "metric-card",
                        span { class: "metric-label", "Merges (1h)" }
                        div { class: "metric-value mono", "{tp.merges_last_hour}" }
                        span { class: "metric-sub", "Last 60 minutes" }
                    }
                    div { class: "metric-card",
                        span { class: "metric-label", "Since Last Merge" }
                        div { class: "metric-value mono",
                            if let Some(m) = tp.minutes_since_last_merge {
                                "{m}m"
                            } else {
                                "-"
                            }
                        }
                        span { class: "metric-sub",
                            if tp.main_idle { "Main branch idle" } else { "Activity active" }
                        }
                    }
                }

                // Recent CI Runs
                if ci.data.is_empty() {
                    div { class: "empty-state",
                        "No recent CI terminal events detected."
                    }
                } else {
                    table { class: "data-table",
                        thead {
                            tr {
                                th { "Status" }
                                th { "Spec" }
                                th { "Timestamp" }
                                th { "Run UUID" }
                            }
                        }
                        tbody {
                            for item in &ci.data {
                                {
                                    let time_str = item.ts.format("%H:%M:%S UTC").to_string();
                                    let short_id = match &item.run_id {
                                        Some(id) if id.len() > 12 => &id[0..12],
                                        Some(id) => id.as_str(),
                                        None => "-",
                                    };
                                    rsx! {
                                        tr {
                                            td {
                                                if item.is_green {
                                                    span { class: "badge badge-green", "✓ Green" }
                                                } else {
                                                    span { class: "badge badge-shelved", "✗ Red" }
                                                }
                                            }
                                            td {
                                                if let Some(s) = &item.spec {
                                                    span { class: "badge badge-spec mono", "{s}" }
                                                } else {
                                                    span { style: "color: var(--text-muted);", "-" }
                                                }
                                            }
                                            td { class: "mono", style: "font-size: 11px;",
                                                "{time_str}"
                                            }
                                            td { class: "mono", style: "font-size: 11px; color: var(--text-muted);",
                                                "{short_id}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "panel-footer",
                span { "Merge throughput from aida integrate; CI state from feed CiTerminal events" }
                span { class: "mono", "Source: {throughput.source_command}" }
            }
        }
    }
}
