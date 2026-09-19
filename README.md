# aida-monitor

A high-fidelity, read-only operator dashboard for [AIDA](https://github.com/joemooney/aida)-managed software projects built in Rust with [Dioxus](https://dioxuslabs.com/).

AIDA runs autonomous coding agents ("drains") that take specifications through implement → CI → review → merge. The operator previously monitored this with terminal loops (`while :; do clear; <command>; sleep N; done`); `aida-monitor` replaces this with a real-time operations dashboard.

---

## Prime Directives

Four core rules govern `aida-monitor`. Violating any of them is considered a defect:

1. **Never write anything.** No file in the target project, no store object, no git operation, no `gh` mutation. This is a display surface, not a control interface.
2. **Never parse human output.** Every AIDA command is invoked with `--format json` or `--json`. If a required number is absent from JSON output, it is treated as an upstream gap, not a text-parsing task.
3. **Never read `.aida-store/` or the SQLite cache directly.** Internal stores are private to AIDA and subject to format changes. All queries go through documented CLI commands or the public event feed.
4. **Degrade, never crash.** Missing fields, non-zero exits, or network drops render quiet staleness markers and `n/a` values. A dashboard running at 3am must remain resilient even when individual panels are degraded.

---

## The 7 Priority Panels

Built in measured priority order targeting the drain round trip bottleneck:

1. **Rounds in Flight (Priority #1):**
   - Per spec currently moving: number of times shelved, current round, one-line reason for most recent shelve, and elapsed duration in the current round.
   - Derived directly from `.aida/events.jsonl` append-only stream.
2. **Phase Clock:**
   - Active phases (`implementer`, `ci`, `reviewer`, `merge`, `pull`, `build`) and elapsed duration compared against empirical median thresholds (e.g. CI wait median ~15m; warns if sitting far past median).
3. **At the Gate:**
   - Supervised PRs held for human review with age and verdict (`aida merge-hold list --json`).
4. **Shelve Causes (Rolling 24h):**
   - Visual segmented proportion bar and counts by cause: reviewer change-requests, red CI, tool failure, and CI timeouts with operational diagnostics.
5. **Queue and Lock:**
   - Whether the drain lock is held, by which PID, and for how long. Active session roster and queue depth per role (`aida ps --json`).
6. **CI & Integration Throughput:**
   - Recent CI runs with green/red conclusion badges and merge throughput metrics (merges in last 24h, last hour, minutes since last merge).
7. **Seat Responsiveness & Schedule:**
   - Unread mail per seat, findings awaiting human triage, and scheduled cron jobs due or overdue.

---

## Polling Discipline

- **Event feed as change signal:** Tails `.aida/events.jsonl` incrementally with file-offset seeking.
- **Staggered command polling:** Slower surfaces poll between 10s and 45s so commands never fire simultaneously. Never polls faster than 5s.
- **Zero hot-path network calls:** CI and status queries run locally or offline-first (`--no-ci` fast paths).

---

## Correctness & Traceability (`--self-check`)

Every metric on screen is traceable to an underlying command or event feed record. Run the built-in self-check suite:

```bash
aida-monitor --self-check
```

The self-check executes each panel's backing command, validates the JSON output against the model, and exits non-zero on any mismatch.

---

## Usage

### 1. Launch Dioxus Desktop Dashboard (GUI)

```bash
# Monitor current directory
cargo run

# Monitor another AIDA project
cargo run -- --path /path/to/project
```

### 2. Plain Terminal Snapshot (`--once`)

Outputs a formatted ANSI terminal snapshot to stdout and exits immediately (ideal for scripts or one-off checks):

```bash
cargo run -- --once
cargo run -- --path /home/joe/ai/aida --once
```

### 3. Interactive Scrolling Terminal Dashboard (`--plain`)

Runs continuous auto-refreshing text output without GUI (ideal for tmux panes or remote SSH sessions):

```bash
cargo run -- --plain --interval 10
```

### 4. Verify Correctness (`--self-check`)

```bash
cargo run -- --self-check
cargo run -- --path /home/joe/ai/aida --self-check
```

---

## Non-Goals

- Any write, dispatch, or control action.
- Replacing AIDA's internal terminal UI (`aida tui`), which is a working surface for one project. This is an operator wall display.
- Reimplementing AIDA's data model from internal SQLite or git orphans.
