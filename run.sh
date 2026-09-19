#!/usr/bin/env bash
#
# Launch aida-monitor via Cargo, forwarding any CLI options.
#
# Usage:
#   ./run.sh                              # Dioxus desktop dashboard (default)
#   ./run.sh --path /path/to/project      # monitor another AIDA project
#   ./run.sh --once                       # one-off terminal snapshot
#   ./run.sh --plain --interval 10        # scrolling terminal dashboard
#   ./run.sh --self-check                 # verify panel data sources
#
# Works from any subdirectory: the script resolves the repository root from
# its own location before invoking Cargo.
#
# trace:TASK-7 | ai:claude

set -euo pipefail

# Resolve the repository root (this script's directory), following symlinks so
# the script still works when linked onto $PATH.
source="${BASH_SOURCE[0]}"
while [ -L "$source" ]; do
    dir="$(cd -P -- "$(dirname -- "$source")" && pwd)"
    source="$(readlink -- "$source")"
    # A relative symlink target is relative to the link's directory.
    [[ $source != /* ]] && source="$dir/$source"
done
repo_root="$(cd -P -- "$(dirname -- "$source")" && pwd)"

cd -- "$repo_root"

# exec so Cargo replaces this shell: signals (Ctrl-C) and the exit status pass
# straight through to the application.
exec cargo run -- "$@"
