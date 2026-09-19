// trace:STORY-1 | ai:antigravity
// trace:TASK-2 | ai:antigravity
use clap::Parser;
use dioxus::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

mod core;
mod render;
mod ui;

#[derive(Parser, Debug)]
#[command(name = "aida-monitor")]
#[command(author = "Joe Mooney <joe.mooney@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Read-only operator dashboard for AIDA-managed projects", long_about = None)]
struct CliArgs {
    /// Path to the AIDA project root (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Output a single plain text snapshot to stdout and exit
    #[arg(long)]
    once: bool,

    /// Run as plain scrolling text in terminal (no GUI)
    #[arg(long)]
    plain: bool,

    /// Verify number correctness by asserting against raw AIDA JSON outputs
    #[arg(long)]
    self_check: bool,

    /// Polling refresh interval in seconds for plain terminal mode (default 30s, min 5s)
    #[arg(short, long, default_value = "30")]
    interval: u64,
}

fn main() {
    let args = CliArgs::parse();
    let project_path = args.path.canonicalize().unwrap_or(args.path.clone());

    // 1. Self-check mode
    if args.self_check {
        match core::self_check::run_self_check(&project_path) {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                eprintln!("\nSelf-check error: {}", e);
                std::process::exit(1);
            }
        }
    }

    // 2. Single snapshot mode (--once)
    if args.once {
        let mut collector = core::collector::Collector::new(&project_path);
        let snapshot = collector.gather_full_snapshot();
        render::plain_text::render_plain_snapshot(snapshot);
        std::process::exit(0);
    }

    // 3. Plain terminal loop mode (--plain)
    if args.plain {
        let interval_secs = args.interval.max(5);
        println!(
            "Starting AIDA Monitor in plain terminal mode (interval: {}s)...",
            interval_secs
        );
        let mut collector = core::collector::Collector::new(&project_path);
        loop {
            // ANSI clear screen and move cursor to top-left
            print!("\x1B[2J\x1B[1;1H");
            let snapshot = collector.gather_full_snapshot();
            render::plain_text::render_plain_snapshot(snapshot);
            std::thread::sleep(Duration::from_secs(interval_secs));
        }
    }

    // 4. Default: Launch Dioxus Desktop UI
    let path_str = project_path.display().to_string();
    // Pass project path through environment so App root component can initialize
    std::env::set_var("AIDA_MONITOR_PROJECT_PATH", &path_str);

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, WindowBuilder};
        let cfg = Config::new().with_window(
            WindowBuilder::new()
                .with_title("AIDA Monitor — Operator Dashboard")
                .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1380.0, 880.0)),
        );

        dioxus::LaunchBuilder::desktop()
            .with_cfg(cfg)
            .launch(RootApp);
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(RootApp);
    }
}

#[component]
fn RootApp() -> Element {
    let initial_path =
        std::env::var("AIDA_MONITOR_PROJECT_PATH").unwrap_or_else(|_| ".".to_string());
    rsx! {
        ui::app::App {
            initial_path,
        }
    }
}
