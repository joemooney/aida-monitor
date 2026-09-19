// trace:STORY-1 | ai:antigravity
use crate::core::models::{GateHeldPr, PanelStatus};
use dioxus::prelude::*;

#[component]
pub fn GatePanel(gate: PanelStatus<Vec<GateHeldPr>>) -> Element {
    rsx! {
        div { class: "panel",
            div { class: "panel-header",
                div { class: "panel-title-area",
                    div { class: "panel-icon-pill icon-rose", "3" }
                    span { class: "panel-title", "At the Gate (Supervised Held PRs)" }
                    span { class: "panel-counter", "{gate.data.len()}" }
                }
                div { class: "panel-meta",
                    span {
                        class: if gate.is_stale { "panel-age stale" } else { "panel-age" },
                        "Updated {gate.age_display()}"
                    }
                }
            }

            div { class: "panel-body",
                if gate.data.is_empty() {
                    div { class: "empty-state",
                        span { style: "color: var(--color-emerald); font-weight: 600;", "Gate is clear. " }
                        "Zero supervised PRs are currently held awaiting operator review."
                    }
                } else {
                    table { class: "data-table",
                        thead {
                            tr {
                                th { "PR" }
                                th { "Title" }
                                th { "Held Age" }
                                th { "Verdict" }
                                th { "Operator Status" }
                            }
                        }
                        tbody {
                            for pr in &gate.data {
                                tr {
                                    td { class: "mono", style: "font-weight: 700; color: var(--color-cyan);",
                                        "#{pr.pr_number}"
                                    }
                                    td { style: "max-width: 320px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                        "{pr.title}"
                                    }
                                    td { class: "mono",
                                        "{pr.age_secs / 3600}h {(pr.age_secs % 3600) / 60}m"
                                    }
                                    td {
                                        if let Some(v) = &pr.verdict {
                                            span { class: "badge badge-round", "{v}" }
                                        } else {
                                            span { style: "color: var(--text-muted);", "Pending" }
                                        }
                                    }
                                    td {
                                        if pr.needs_attention {
                                            span { class: "badge badge-shelved", "HELD > 2H (ACTION NEEDED)" }
                                        } else {
                                            span { class: "badge badge-round", "Held (Waves continue)" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "panel-footer",
                span { "Supervised PRs held for human review; waves continue past held PRs" }
                span { class: "mono", "Source: {gate.source_command}" }
            }
        }
    }
}
