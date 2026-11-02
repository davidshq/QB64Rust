# Hooks: the rules Claude Code enforces

`.claude\settings.json` runs `guard_tool.py` before a tool call. It exists because rule text was not followed
(`study\41` §1: staging against human rule 1, deleting instead of `git mv`). It refuses with exit code 2 and its
reason goes to Claude, never to the user. It enforces human rules and the layout of `target\`; nothing here
refuses a test run or holds a session (the close gate and the Stop hook went with the OpenSpec change
`testing-reset`: `tools\check\` runs tests and gates nothing).

| Hook | When | Refuses |
|---|---|---|
| `guard_tool.py` | before Bash, PowerShell, Write, Edit | staging, stashing, resetting (`git add`, `git stash`, `git reset`, `git restore --staged`, `git rm --cached`, `git commit -a`); a plain `mv`/`rm`/`Move-Item`/`Remove-Item` of a tracked file (use `git mv`); a file written at the top of `target\` or under it outside `target\scratch\` and `target\check\`; any write under `..\QB64pe\` |

`common.py` holds what a hook needs: the repo root (`CLAUDE_PROJECT_DIR`), reading the input, refusing.

Tests: `python -m unittest discover -s tools/hooks -p "test_*.py"` (and the `tools` step of `just quick`).

Known limits: `guard_tool.py` reads the command's text, so a command that only mentions a staging command (a
`grep "git add"`, an `echo`) is refused too, and `git commit <path>` (which stages the path) and a `tee` into
`target\` are not caught.

To stop the hook for one session, remove its entry from `.claude\settings.json` locally (the user's call); there is
no flag Claude can pass.
