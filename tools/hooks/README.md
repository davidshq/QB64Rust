# Hooks: the rules Claude Code enforces

`.claude\settings.json` runs these before a tool call and when Claude is about to stop. They exist because rule text
was not followed (`study\41` §1: staging against human rule 1, deleting instead of `git mv`, closes run per task
group, stopping after each task group). A hook refuses with exit code 2 and its reason goes to Claude, never to the
user. Nothing here changes what is tested; it changes when, and what Claude cannot do.

| Hook | When | Refuses |
|---|---|---|
| `guard_tool.py` | before Bash, PowerShell, Write, Edit | staging, stashing, resetting (`git add`, `git stash`, `git reset`, `git restore --staged`, `git rm --cached`, `git commit -a`); a plain `mv`/`rm`/`Move-Item`/`Remove-Item` of a tracked file (use `git mv`); a file written at the top of `target\` or under it outside `target\scratch\` and `target\close\`; any write under `..\QB64pe\`; a closing run (`close.py --plan`/`--check`, a `--list` or `--pass-cache` run, the call-site check, the sandbox or Linux family) while the active change has open tasks |
| `stop_continue.py` | when Claude ends its turn | the stop, while a change worked on in the last day has unchecked tasks and the message neither asks the user anything nor hands over the files to stage nor says Claude is blocked; at most 25 times a session |

`common.py` holds what they share: the repo root (`CLAUDE_PROJECT_DIR`), the open tasks (unchecked `- [ ]` lines of
`openspec\changes\*\tasks.md`, archive left out, the closing task left out, a change untouched for a day left out),
and the state folder `target\scratch\hooks\`.

Tests: `python -m unittest discover -s tools/hooks -p "test_*.py"` (CI, `rust.yml`).

Known limits: `guard_tool.py` reads the command's text, so a command that only mentions a staging command (a
`grep "git add"`, an `echo`) is refused too, and `git commit <path>` (which stages the path) and a `tee` into
`target\` are not caught. `stop_continue.py` lets a stop through when the input has no `last_assistant_message`
(it cannot then tell a question from a stop mid-change).

To stop a hook for one session, remove its entry from `.claude\settings.json` locally (the user's call); there is no
flag Claude can pass.
