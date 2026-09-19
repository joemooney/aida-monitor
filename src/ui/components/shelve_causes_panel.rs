// trace:STORY-1 | ai:antigravity
use crate::core::models::{PanelStatus, ShelveCauses24h};
use dioxus::prelude::*;

#[component]
pub fn ShelveCausesPanel(shelve_causes: PanelStatus<ShelveCauses24h>) -> Element {
    let data = &shelve_causes.data;

    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-indigo", "4" }
                    span { class: "panel-title", "Shelve Causes (Rolling 24h)" }
                    span { class: "panel-counter", "{data.total_shelves} total" }
                }
                div { class: "panel-meta",
                    span {
                        class: if shelve_causes.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {shelve_causes.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                if data.total_shelves == 0 {
                    div { class: "empty-state",
                        span { style: "color: var(--color-emerald); font-weight: 600;", "Zero shelves in last 24h. " }
                        "Specs are shipping without round interruptions."
                    }
                } else {
                    // Segmented proportion bar
                    div { class: "stacked-bar-container",
                        for (cause, count) in &data.counts_by_cause {
                            {
                                let pct = data.percentages.get(cause).copied().unwrap_or(0.0);
                                let bg_color = match cause.as_str() {
                                    "reviewer-changes" => "#818cf8",
                                    "ci-red" => "#f43f5e",
                                    "ci-unavailable" => "#f59e0b",
                                    "tool-exit" => "#ec4899",
                                    "timeout" => "#a855f7",
                                    _ => "#06b6d4",
                                };
                                rsx! {
                                    div {
                                        class: "stacked-bar-segment",
                                        style: "width: {pct}%; background-color: {bg_color};",
                                        title: "{cause}: {count} ({pct}%)"
                                    }
                                }
                            }
                        }
                    }

                    // Legend and counts
                    div { class: "cause-legend",
                        for (cause, count) in &data.counts_by_cause {
                            {
                                let pct = data.percentages.get(cause).copied().unwrap_or(0.0);
                                let dot_color = match cause.as_str() {
                                    "reviewer-changes" => "#818cf8",
                                    "ci-red" => "#f43f5e",
                                    "ci-unavailable" => "#f59e0b",
                                    "tool-exit" => "#ec4899",
                                    "timeout" => "#a855f7",
                                    _ => "#06b6d4",
                                };
                                rsx! {
                                    div { class: "cause-legend-item",
                                        span { class: "legend-dot", style: "background-color: {dot_color};" }
                                        span { class: "mono", style: "font-weight: 600;", "{cause}:" }
                                        span { style: "color: var(--text-primary); font-weight: 700;", "{count}" }
                                        span { style: "color: var(--text-muted); font-size: 11px;", "({pct}%)" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "diagnostic-callout",
                        span { style: "font-weight: 700; margin-right: 6px;", "Operator Insight:" }
                        "{data.primary_diagnostic}"
                    }
                }
            }

            div { class: "panel-footer",
                span { "Derived from SpecShelved events in .aida/events.jsonl over the past 24h" }
                span { class: "mono", "Source: {shelve_causes.source_command}" }
            }
        }
    }
}
