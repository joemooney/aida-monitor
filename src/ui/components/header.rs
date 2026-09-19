// trace:STORY-1 | ai:antigravity
use dioxus::prelude::*;

#[component]
pub fn Header(
    project_name: String,
    project_path: String,
    active_wave: Option<String>,
    lock_held: bool,
    active_tab: Signal<String>,
    search_query: Signal<String>,
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
                        span { "FEED TAILING ACTIVE" }
                    }

                    if lock_held {
                        span { class: "badge badge-shelved", "LOCK HELD" }
                    } else {
                        span { class: "badge badge-green", "LOCK FREE" }
                    }

                    if let Some(wave) = active_wave {
                        span { class: "badge badge-round", "Wave: {wave}" }
                    }

                    button {
                        class: "action-btn",
                        onclick: move |_| on_refresh.call(()),
                        "⟳ Refresh"
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
