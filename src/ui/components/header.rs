// trace:STORY-1 | ai:antigravity
// trace:TASK-2 | ai:antigravity
// trace:STORY-10 | ai:antigravity
use crate::core::models::ApiGuardState;
use dioxus::prelude::*;

#[component]
pub fn Header(
    project_name: String,
    project_path: String,
    active_wave: Option<String>,
    lock_held: bool,
    active_tab: Signal<String>,
    search_query: Signal<String>,
    interval_secs: Signal<u64>,
    zoom_level: Signal<f32>,
    api_guard: ApiGuardState,
    is_refreshing: bool,
    on_refresh: EventHandler<()>,
) -> Element {
    rsx! {
        header { class: "top-header",
            div { class: "header-row-1",
                div { class: "brand-section",
                    div { class: "brand-badge", "AIDA" }
                    span { class: "brand-title", "OPERATOR MONITOR" }
                    div { class: "project-tag",
                        span { class: "project-name-pill", "{project_name}" }
                        span { class: "mono", style: "font-size: 11px; opacity: 0.7;", "{project_path}" }
                    }
                }

                div { class: "header-status-ribbon",
                    div { class: "pulse-indicator",
                        span { class: "pulse-dot" }
                        span { "FEED TAIL ACTIVE" }
                    }

                    div {
                        class: "api-guard-pill",
                        title: "{api_guard.policy}",
                        span { class: "shield-icon", "🛡" }
                        span { "API: Safe (0 calls)" }
                    }

                    div { class: "interval-selector",
                        span { class: "interval-label", "Poll:" }
                        div { class: "interval-group",
                            button {
                                class: if interval_secs() == 30 { "interval-btn active" } else { "interval-btn" },
                                onclick: move |_| interval_secs.set(30),
                                "30s"
                            }
                            button {
                                class: if interval_secs() == 60 { "interval-btn active" } else { "interval-btn" },
                                onclick: move |_| interval_secs.set(60),
                                "1m"
                            }
                            button {
                                class: if interval_secs() == 120 { "interval-btn active" } else { "interval-btn" },
                                onclick: move |_| interval_secs.set(120),
                                "2m"
                            }
                            button {
                                class: if interval_secs() == 0 { "interval-btn active" } else { "interval-btn" },
                                onclick: move |_| interval_secs.set(0),
                                "Pause"
                            }
                        }
                    }

                    {
                        let zoom_pct = (zoom_level() * 100.0).round() as u32;
                        rsx! {
                            button {
                                class: if zoom_pct != 100 { "zoom-pill active mono" } else { "zoom-pill mono" },
                                title: "Zoom: Ctrl + Scroll or Ctrl +/-. Click to reset (100%).",
                                onclick: move |_| zoom_level.set(1.0),
                                "🔍 {zoom_pct}%"
                                if zoom_pct != 100 {
                                    span { style: "font-size: 10px; margin-left: 2px;", "↺" }
                                }
                            }
                        }
                    }

                    if lock_held {
                        span { class: "badge badge-shelved", "LOCK HELD" }
                    } else {
                        span { class: "badge badge-green", "LOCK FREE" }
                    }

                    if let Some(wave) = active_wave {
                        span { class: "badge badge-round", "Wave: {wave}" }
                    }

                    if is_refreshing {
                        button {
                            class: "action-btn disabled",
                            disabled: true,
                            "⟳ Wait..."
                        }
                    } else {
                        button {
                            class: "action-btn",
                            onclick: move |_| on_refresh.call(()),
                            "⟳ Refresh"
                        }
                    }
                }
            }

            div { class: "header-row-2",
                nav { class: "tab-nav",
                    button {
                        class: if active_tab() == "overview" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("overview".to_string()),
                        "Overview (7 Panels)"
                    }
                    button {
                        class: if active_tab() == "rounds" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("rounds".to_string()),
                        "Rounds in Flight (#1)"
                    }
                    button {
                        class: if active_tab() == "phases" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("phases".to_string()),
                        "Phase Clock & CI"
                    }
                    button {
                        class: if active_tab() == "gate" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("gate".to_string()),
                        "Gate & Queues"
                    }
                    button {
                        class: if active_tab() == "events" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("events".to_string()),
                        "Event Stream"
                    }
                    button {
                        class: if active_tab() == "self_check" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("self_check".to_string()),
                        "✓ Self-Check Mode"
                    }
                }

                div { class: "search-and-tools",
                    input {
                        class: "search-input mono",
                        r#type: "text",
                        placeholder: "Filter spec (e.g. TASK-1291)...",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value()),
                    }
                }
            }
        }
    }
}
