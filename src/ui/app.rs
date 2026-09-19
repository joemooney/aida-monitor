// trace:STORY-1 | ai:antigravity
use super::components::ci_panel::CiPanel;
use super::components::event_drawer::EventDrawer;
use super::components::gate_panel::GatePanel;
use super::components::header::Header;
use super::components::phase_clock_panel::PhaseClockPanel;
use super::components::queue_lock_panel::QueueLockPanel;
use super::components::rounds_panel::RoundsPanel;
use super::components::seat_panel::SeatPanel;
use super::components::self_check_view::SelfCheckView;
use super::components::shelve_causes_panel::ShelveCausesPanel;
use super::theme::DASHBOARD_CSS;
use crate::core::collector::Collector;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Props, PartialEq)]
pub struct AppProps {
    pub initial_path: String,
}

#[component]
pub fn App(props: AppProps) -> Element {
    let initial_path = props.initial_path.clone();
    let collector_cell = use_signal(|| {
        let mut c = Collector::new(&initial_path);
        let snap = c.gather_full_snapshot().clone();
        (Arc::new(Mutex::new(c)), snap)
    });

    let mut snapshot = use_signal(|| collector_cell().1.clone());
    let active_tab = use_signal(|| "overview".to_string());
    let search_query = use_signal(|| "".to_string());

    let collector_arc = collector_cell().0.clone();

    // Background asynchronous polling loop (respecting polling discipline)
    use_future(move || {
        let col = collector_arc.clone();
        async move {
            let mut tick_count = 0u64;
            loop {
                tokio::time::sleep(Duration::from_millis(1500)).await;
                tick_count += 1;

                if let Ok(mut guard) = col.lock() {
                    // 1. Fast event feed tail (every 1.5s)
                    let _ = guard.refresh_feed();

                    // 2. Medium commands: ps & drain status (every 9s / 6 ticks)
                    if tick_count.is_multiple_of(6) {
                        guard.refresh_queue_and_lock();
                    }

                    // 3. At the gate: merge holds (every 18s / 12 ticks)
                    if tick_count.is_multiple_of(12) {
                        guard.refresh_merge_holds();
                    }

                    // 4. Slower commands: awaiting, schedule, integrate (every 30s / 20 ticks)
                    if tick_count.is_multiple_of(20) {
                        guard.refresh_seats();
                        guard.refresh_integrate();
                    }

                    let new_snap = guard.last_snapshot.clone();
                    snapshot.set(new_snap);
                }
            }
        }
    });

    let current = snapshot();
    let col_clone = collector_cell().0.clone();

    let on_refresh = move |_| {
        if let Ok(mut guard) = col_clone.lock() {
            let new_snap = guard.gather_full_snapshot().clone();
            snapshot.set(new_snap);
        }
    };

    let active_wave = current.queue_lock.data.active_wave.clone();
    let lock_held = current.queue_lock.data.lock_held;
    let rounds_in_flight_count = current.rounds.data.len();
    let held_prs_count = current.gate.data.len();
    let queue_depth_count = current.queue_lock.data.total_queue;
    let shelves_24h_count = current.shelve_causes.data.total_shelves;
    let merges_today_count = current.throughput.data.merges_last_day;

    rsx! {
        style { "{DASHBOARD_CSS}" }

        div { class: "app-container",
            Header {
                project_name: current.project_name.clone(),
                project_path: current.project_path.clone(),
                active_wave,
                lock_held,
                active_tab,
                search_query,
                on_refresh,
            }

            // Top Quick Metrics Ribbon
            div { class: "metric-ribbon",
                div { class: "metric-card",
                    span { class: "metric-label", "Rounds in Flight" }
                    div { class: "metric-value mono", style: if rounds_in_flight_count > 0 { "color: var(--color-cyan);" } else { "color: var(--text-muted);" },
                        "{rounds_in_flight_count}"
                    }
                    span { class: "metric-sub", "Moving specs (Priority #1)" }
                }

                div { class: "metric-card",
                    span { class: "metric-label", "At The Gate" }
                    div { class: "metric-value mono", style: if held_prs_count > 0 { "color: var(--color-amber);" } else { "color: var(--color-emerald);" },
                        "{held_prs_count}"
                    }
                    span { class: "metric-sub", "Supervised PRs held" }
                }

                div { class: "metric-card",
                    span { class: "metric-label", "Queue Depth" }
                    div { class: "metric-value mono", "{queue_depth_count}" }
                    span { class: "metric-sub", "Active work items" }
                }

                div { class: "metric-card",
                    span { class: "metric-label", "Shelves (Rolling 24h)" }
                    div { class: "metric-value mono", style: if shelves_24h_count > 5 { "color: var(--color-rose);" } else { "color: var(--text-primary);" },
                        "{shelves_24h_count}"
                    }
                    span { class: "metric-sub", "Round interruptions" }
                }

                div { class: "metric-card",
                    span { class: "metric-label", "Merges (Past 24h)" }
                    div { class: "metric-value mono", style: "color: var(--color-emerald);",
                        "{merges_today_count}"
                    }
                    span { class: "metric-sub", "Shipped pull requests" }
                }
            }

            // Tab View Rendering
            if active_tab() == "overview" {
                div { class: "dashboard-grid",
                    div { class: "panel-col-8",
                        RoundsPanel {
                            rounds: current.rounds.clone(),
                            filter: search_query().clone(),
                        }
                    }
                    div { class: "panel-col-4",
                        ShelveCausesPanel {
                            shelve_causes: current.shelve_causes.clone(),
                        }
                    }
                    div { class: "panel-col-6",
                        PhaseClockPanel {
                            phases: current.phases.clone(),
                            filter: search_query().clone(),
                        }
                    }
                    div { class: "panel-col-6",
                        GatePanel {
                            gate: current.gate.clone(),
                        }
                    }
                    div { class: "panel-col-6",
                        QueueLockPanel {
                            queue_lock: current.queue_lock.clone(),
                        }
                    }
                    div { class: "panel-col-6",
                        CiPanel {
                            ci: current.ci.clone(),
                            throughput: current.throughput.clone(),
                        }
                    }
                    div { class: "panel-col-12",
                        SeatPanel {
                            seats: current.seats.clone(),
                        }
                    }
                }
            } else if active_tab() == "rounds" {
                div { class: "dashboard-grid",
                    div { class: "panel-col-12",
                        RoundsPanel {
                            rounds: current.rounds.clone(),
                            filter: search_query().clone(),
                        }
                    }
                    div { class: "panel-col-12",
                        EventDrawer {
                            events: current.recent_events.clone(),
                            filter: search_query().clone(),
                        }
                    }
                }
            } else if active_tab() == "phases" {
                div { class: "dashboard-grid",
                    div { class: "panel-col-6",
                        PhaseClockPanel {
                            phases: current.phases.clone(),
                            filter: search_query().clone(),
                        }
                    }
                    div { class: "panel-col-6",
                        CiPanel {
                            ci: current.ci.clone(),
                            throughput: current.throughput.clone(),
                        }
                    }
                }
            } else if active_tab() == "gate" {
                div { class: "dashboard-grid",
                    div { class: "panel-col-6",
                        GatePanel {
                            gate: current.gate.clone(),
                        }
                    }
                    div { class: "panel-col-6",
                        QueueLockPanel {
                            queue_lock: current.queue_lock.clone(),
                        }
                    }
                    div { class: "panel-col-12",
                        SeatPanel {
                            seats: current.seats.clone(),
                        }
                    }
                }
            } else if active_tab() == "events" {
                div { class: "dashboard-grid",
                    div { class: "panel-col-12",
                        EventDrawer {
                            events: current.recent_events.clone(),
                            filter: search_query().clone(),
                        }
                    }
                }
            } else if active_tab() == "self_check" {
                SelfCheckView {
                    snapshot: current.clone(),
                }
            }
        }
    }
}
