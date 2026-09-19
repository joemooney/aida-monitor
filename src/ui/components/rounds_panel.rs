// trace:STORY-1 | ai:antigravity
use crate::core::models::{PanelStatus, RoundInFlight};
use dioxus::prelude::*;

#[component]
pub fn RoundsPanel(rounds: PanelStatus<Vec<RoundInFlight>>, filter: String) -> Element {
    let filtered: Vec<_> = rounds
        .data
        .iter()
        .filter(|r| {
            if filter.is_empty() {
                true
            } else {
                r.spec.to_lowercase().contains(&filter.to_lowercase())
            }
        })
        .collect();

    rsx! {
        div { class: "panel highlight-priority",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-cyan", "1" }
                    span { class: "panel-title", "Rounds in Flight" }
                    span { class: "panel-counter", "{filtered.len()}" }
                }
                div { class: "panel-meta",
                    span {
                        class: if rounds.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {rounds.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                if filtered.is_empty() {
                    div { class: "empty-state",
                        if filter.is_empty() {
                            "No specs currently in flight. Drains are either idle or between rounds."
                        } else {
                            "No in-flight specs match the search filter."
                        }
                    }
                } else {
                    div { class: "rounds-list",
                        for r in filtered {
                            div { class: "round-card",
                                div { class: "round-header",
                                    div { class: "round-header-left",
                                        span { class: "badge badge-spec mono", "{r.spec}" }
                                        span { class: "badge badge-round", "Round {r.round}" }
                                        if r.shelved_count > 0 {
                                            span {
                                                class: if r.shelved_count > 1 { "badge badge-shelved" } else { "badge badge-round" },
                                                "{r.shelved_count}x Shelved"
                                            }
                                        } else {
                                            span { class: "badge badge-green", "Clean Run" }
                                        }
                                        if let Some(phase) = &r.current_phase {
                                            span { class: "badge badge-phase", "Phase: {phase}" }
                                        }
                                    }
                                    div { class: "round-timer mono",
                                        span { style: "color: var(--text-muted); font-size: 11px;", "In round:" }
                                        span {
                                            style: if r.elapsed_round_secs > 1800 { "color: var(--color-amber);" } else { "color: var(--text-primary);" },
                                            "{r.elapsed_round_secs / 60}m {r.elapsed_round_secs % 60}s"
                                        }
                                    }
                                }

                                if let Some(cause) = &r.last_shelve_cause {
                                    div { class: "round-reason-box",
                                        div { class: "round-reason-title mono", "Last shelve cause: {cause}" }
                                        if let Some(detail) = &r.last_shelve_detail {
                                            div { class: "round-reason-detail", "{detail}" }
                                        }
                                        if let Some(hint) = &r.last_shelve_recovery_hint {
                                            div { class: "round-hint mono", "↳ Recovery: {hint}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "panel-footer",
                span { "Derived from .aida/events.jsonl (SpecShelved & SpecRetried events)" }
                span { class: "mono", "Source: {rounds.source_command}" }
            }
        }
    }
}
