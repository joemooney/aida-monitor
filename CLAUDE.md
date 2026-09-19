# CLAUDE.md

Guidance for Claude Code working in this repository. AIDA conventions
(trace format, commit format, daily commands, capture rules) live in
`.claude/AIDA.md` — Claude Code expands the import below automatically,
so you'll see them in context without this file having to duplicate
them.

@.claude/AIDA.md

<!-- AIDA-MEMORY-REFLEX-BEGIN -->
## AIDA Memory Reflex

This project uses AIDA as a quiet requirements store and project memory.
Use the CLI in compact agent mode for routine reads:

```bash
AIDA_AGENT_OUTPUT=toon aida search "<topic>"
AIDA_AGENT_OUTPUT=toon aida show <SPEC-ID>
AIDA_AGENT_OUTPUT=toon aida list --status approved
```

Before making a meaningful change, search for related requirements or
decisions. When you learn something that should outlive the chat, add or
update memory as you go:

```bash
aida add --type task --status draft --title "..."
aida comment add <SPEC-ID> "..."
aida edit <SPEC-ID> --status in-progress
```

TOON output includes freshness metadata such as `modified_at`; verify old
memory before relying on it. Prefer the `$aida-memory-query` and
`$aida-memory-capture` skills when they are available.
<!-- AIDA-MEMORY-REFLEX-END -->


## Project overview

aida-monitor

## Discipline for AIDA-using sessions

@.aida/discipline/README.md

