// trace:STORY-1 | ai:antigravity
use crate::core::models::EventItem;
use dioxus::prelude::*;

#[component]
pub fn EventDrawer(events: Vec<EventItem>, filter: String) -> Element {
    let filtered: Vec<_> = events
        .iter()
        .filter(|e| {
            if filter.is_empty() {
                true
            } else if let Some(spec) = &e.spec {
                spec.to_lowercase().contains(&filter.to_lowercase())
            } else {
                false
            }
        })
        .collect();

    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-cyan", "▶" }
                    span { class: "panel-title", "Real-Time Event Feed (.aida/events.jsonl)" }
                    span { class: "panel-counter", "{filtered.len()} events" }
                }
                div { class: "panel-meta",
                    span { class: "pulse-indicator",
                        span { class: "pulse-dot" }
                        "TAILING APPEND-ONLY STREAM"
                    }
                }
            }

            div { class: "panel-body",
                div { class: "event-feed-box mono",
                    if filtered.is_empty() {
                        div { class: "empty-state",
                            "No events in feed matching filter. Feed is tailed directly from .aida/events.jsonl."
                        }
                    } else {
                        for ev in filtered {
                            {
                                let event_name = ev.kind.get("event").and_then(|v| v.as_str()).unwrap_or("Event");
                                let badge_style = match event_name {
                                    "SpecShelved" => "badge-shelved",
                                    "SpecRetried" => "badge-round",
                                    "PrMerged" | "PhaseDonePr" => "badge-green",
                                    "PhaseEntered" => "badge-phase",
                                    _ => "badge-round",
                                };
                                let time_str = ev.ts.format("%H:%M:%S").to_string();
                                rsx! {
                                    div { class: "event-line",
                                        span { style: "color: var(--text-muted); font-size: 11px;",
                                            "{time_str}"
                                        }
                                        span { class: "badge {badge_style}", "{event_name}" }
                                        if let Some(spec) = &ev.spec {
                                            span { class: "badge badge-spec", "{spec}" }
                                        }
                                        span { style: "color: var(--text-secondary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1;",
                                            "{ev.kind}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "panel-footer",
                span { "Raw append-only JSONL stream; forward-compatible (unknown events logged & ignored)" }
                span { class: "mono", "Direct file tailing" }
            }
        }
    }
}
