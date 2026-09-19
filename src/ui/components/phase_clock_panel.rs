// trace:STORY-1 | ai:antigravity
use crate::core::models::{PanelStatus, PhaseClockItem};
use dioxus::prelude::*;

#[component]
pub fn PhaseClockPanel(phases: PanelStatus<Vec<PhaseClockItem>>, filter: String) -> Element {
    let filtered: Vec<_> = phases
        .data
        .iter()
        .filter(|p| {
            if filter.is_empty() {
                true
            } else if let Some(spec) = &p.spec {
                spec.to_lowercase().contains(&filter.to_lowercase())
            } else {
                true
            }
        })
        .collect();

    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-amber", "2" }
                    span { class: "panel-title", "Phase Clock" }
                    span { class: "panel-counter", "{filtered.len()}" }
                }
                div { class: "panel-meta",
                    span {
                        class: if phases.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {phases.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                if filtered.is_empty() {
                    div { class: "empty-state",
                        "No sessions currently in active phase execution."
                    }
                } else {
                    div { class: "phase-items-list",
                        for p in filtered {
                            {
                                let pct = ((p.elapsed_secs as f64 / p.median_secs as f64) * 100.0).min(200.0);
                                let bar_width = (pct / 2.0).min(100.0);
                                let fill_class = if p.is_over_median {
                                    "phase-progress-fill progress-rose"
                                } else if pct > 75.0 {
                                    "phase-progress-fill progress-amber"
                                } else {
                                    "phase-progress-fill progress-green"
                                };

                                rsx! {
                                    div { class: "phase-row", style: "flex-direction: column; align-items: stretch;",
                                        div { style: "display: flex; justify-content: space-between; align-items: center;",
                                            div { style: "display: flex; align-items: center; gap: 8px;",
                                                if let Some(spec) = &p.spec {
                                                    span { class: "badge badge-spec mono", "{spec}" }
                                                }
                                                span { class: "badge badge-phase", "{p.phase}" }
                                                if let Some(seat) = &p.seat {
                                                    span { style: "font-size: 11px; color: var(--text-muted);", "seat: {seat}" }
                                                }
                                            }

                                            div { class: "mono", style: "font-size: 12px; font-weight: 600;",
                                                span {
                                                    style: if p.is_over_median { "color: var(--color-rose); font-weight: 700;" } else { "color: var(--text-primary);" },
                                                    "{p.elapsed_secs / 60}m {p.elapsed_secs % 60}s"
                                                }
                                                span { style: "color: var(--text-muted); font-size: 11px; margin-left: 6px;",
                                                    "(median: {p.median_secs / 60}m)"
                                                }
                                                if p.is_over_median {
                                                    span { class: "badge badge-shelved", style: "margin-left: 8px;", "OVER MEDIAN" }
                                                }
                                            }
                                        }

                                        div { class: "phase-progress-bar",
                                            div {
                                                class: "{fill_class}",
                                                style: "width: {bar_width}%;"
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
                span { "Derived from consecutive PhaseEntered timestamps in .aida/events.jsonl" }
                span { class: "mono", "Source: {phases.source_command}" }
            }
        }
    }
}
