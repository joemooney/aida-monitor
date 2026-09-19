// trace:STORY-1 | ai:antigravity
// trace:TASK-2 | ai:antigravity
use crate::core::models::DashboardSnapshot;
use colored::Colorize;

pub fn render_plain_snapshot(snapshot: &DashboardSnapshot) {
    println!(
        "{}",
        "================================================================================".cyan()
    );
    println!(
        " {}  {}   {}",
        "AIDA OPERATOR MONITOR".bold().white(),
        format!("[{}]", snapshot.project_name).yellow().bold(),
        snapshot
            .timestamp
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
            .cyan()
    );
    println!(" Path:   {}", snapshot.project_path.dimmed());
    println!(
        " Safety: {} [0 GitHub calls / immune to rate limits]",
        "OFFLINE-FIRST".green().bold()
    );
    println!(
        "{}",
        "================================================================================".cyan()
    );

    // 1. Rounds in Flight
    println!(
        "\n{}",
        format!(
            "▶ 1. ROUNDS IN FLIGHT ({}) — [{}]",
            snapshot.rounds.data.len(),
            snapshot.rounds.age_display()
        )
        .bold()
        .cyan()
    );
    if snapshot.rounds.data.is_empty() {
        println!(
            "   {}",
            "No active rounds moving in this project window.".dimmed()
        );
    } else {
        println!(
            "   {:<12} {:<9} {:<12} {:<16} {:<12} {:<24}",
            "SPEC", "ROUND", "SHELVED", "CURRENT PHASE", "TIME IN ROUND", "LAST SHELVE REASON"
        );
        println!(
            "   {}",
            "--------------------------------------------------------------------------------"
                .dimmed()
        );
        for r in &snapshot.rounds.data {
            let phase_str = r.current_phase.as_deref().unwrap_or("idle");
            let round_time = format!(
                "{}m {}s",
                r.elapsed_round_secs / 60,
                r.elapsed_round_secs % 60
            );
            let shelve_reason = r.last_shelve_cause.as_deref().unwrap_or("none");
            let spec_styled = if r.shelved_count > 1 {
                r.spec.red().bold()
            } else {
                r.spec.green().bold()
            };
            println!(
                "   {:<20} {:<9} {:<12} {:<16} {:<12} {:<24}",
                spec_styled,
                format!("Rnd {}", r.round).yellow(),
                format!("{}x", r.shelved_count),
                phase_str.cyan(),
                round_time,
                shelve_reason.dimmed()
            );
            if let Some(detail) = &r.last_shelve_detail {
                println!("     ↳ Detail: {}", detail.dimmed());
            }
        }
    }

    // 2. Phase Clock
    println!(
        "\n{}",
        format!(
            "▶ 2. PHASE CLOCK ({}) — [{}]",
            snapshot.phases.data.len(),
            snapshot.phases.age_display()
        )
        .bold()
        .cyan()
    );
    if snapshot.phases.data.is_empty() {
        println!(
            "   {}",
            "No sessions currently in active phase execution.".dimmed()
        );
    } else {
        for p in &snapshot.phases.data {
            let spec_str = p.spec.as_deref().unwrap_or("fleet");
            let elapsed_str = format!("{}m {}s", p.elapsed_secs / 60, p.elapsed_secs % 60);
            let median_str = format!("{}m", p.median_secs / 60);
            let status_badge = if p.is_over_median {
                format!("[OVER MEDIAN: {} > {}]", elapsed_str, median_str)
                    .red()
                    .bold()
            } else {
                format!("[RUNNING: {} (median {})]", elapsed_str, median_str).green()
            };
            println!(
                "   • {:<12} Phase: {:<14} Seat: {:<12} {}",
                spec_str.bold(),
                p.phase.yellow(),
                p.seat.as_deref().unwrap_or("-"),
                status_badge
            );
        }
    }

    // 3. At The Gate (Held PRs)
    println!(
        "\n{}",
        format!(
            "▶ 3. AT THE GATE (HELD PRS: {}) — [{}]",
            snapshot.gate.data.len(),
            snapshot.gate.age_display()
        )
        .bold()
        .cyan()
    );
    if snapshot.gate.data.is_empty() {
        println!(
            "   {}",
            "Gate clear. Zero PRs held for human review.".green()
        );
    } else {
        for pr in &snapshot.gate.data {
            let age_str = format!("{}h {}m", pr.age_secs / 3600, (pr.age_secs % 3600) / 60);
            let tag = if pr.needs_attention {
                "[ACTION REQUIRED: >2h]".red().bold()
            } else {
                "[HELD]".yellow()
            };
            println!(
                "   • PR #{:<6} Age: {:<10} {} {}",
                pr.pr_number.to_string().bold(),
                age_str,
                tag,
                pr.title.dimmed()
            );
        }
    }

    // 4. Shelve Causes (Rolling 24h)
    let causes = &snapshot.shelve_causes.data;
    println!(
        "\n{}",
        format!(
            "▶ 4. SHELVE CAUSES (ROLLING 24H: {} TOTAL) — [{}]",
            causes.total_shelves,
            snapshot.shelve_causes.age_display()
        )
        .bold()
        .cyan()
    );
    if causes.total_shelves == 0 {
        println!("   {}", "Zero shelves in last 24h.".green());
    } else {
        for (cause, count) in &causes.counts_by_cause {
            let pct = causes.percentages.get(cause).copied().unwrap_or(0.0);
            println!("   • {:<20} : {:>3} ({:>5.1}%)", cause.bold(), count, pct);
        }
        println!("   ↳ {}", causes.primary_diagnostic.yellow());
    }

    // 5. Queue & Lock
    let q = &snapshot.queue_lock.data;
    let lock_display = if q.lock_held {
        format!("HELD by PID {:?}", q.locked_by_pid).red().bold()
    } else {
        "FREE / UNLOCKED".green().bold()
    };
    println!(
        "\n{}",
        format!(
            "▶ 5. QUEUE & DRAIN LOCK — [{}]",
            snapshot.queue_lock.age_display()
        )
        .bold()
        .cyan()
    );
    println!(
        "   Drain Lock: {} | Active Wave: {}",
        lock_display,
        q.active_wave.as_deref().unwrap_or("none").yellow()
    );
    print!("   Role Depths: ");
    for (role, depth) in &q.role_depths {
        print!("{}: {}  ", role.cyan(), depth);
    }
    println!("(Total queue: {})", q.total_queue);

    // 6. Throughput & CI
    let tp = &snapshot.throughput.data;
    println!(
        "\n{}",
        format!(
            "▶ 6. INTEGRATION & CI THROUGHPUT — [{}]",
            snapshot.throughput.age_display()
        )
        .bold()
        .cyan()
    );
    let last_merge_str = tp
        .minutes_since_last_merge
        .map(|m| format!("{}m ago", m))
        .unwrap_or_else(|| "n/a".to_string());
    println!(
        "   Merges (24h): {:<4} | Merges (1h): {:<4} | Last merge: {:<10} | Main idle: {}",
        tp.merges_last_day.to_string().green().bold(),
        tp.merges_last_hour,
        last_merge_str,
        if tp.main_idle {
            "YES".green()
        } else {
            "NO".red()
        }
    );

    // 7. Seat Responsiveness
    let s = &snapshot.seats.data;
    println!(
        "\n{}",
        format!(
            "▶ 7. SEAT RESPONSIVENESS & SCHEDULE — [{}]",
            snapshot.seats.age_display()
        )
        .bold()
        .cyan()
    );
    println!(
        "   Unread Mail: {:?} | Findings Triage: {} | Scheduled Jobs Due: {} | Overdue: {}",
        s.unread_mail,
        s.findings_total.to_string().yellow(),
        s.due_jobs,
        if s.overdue_jobs > 0 {
            s.overdue_jobs.to_string().red().bold()
        } else {
            "0".green()
        }
    );

    println!(
        "\n{}",
        "--------------------------------------------------------------------------------".dimmed()
    );
    println!(
        " Feed tail: {} total events processed | Degrade, never crash | Read-only mode active\n",
        snapshot.feed_event_count
    );
}
