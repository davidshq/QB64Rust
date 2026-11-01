# The one entry point for testing: tools/check/README.md. Each recipe is one line calling tools/check/check.py.
# Install: cargo install just --locked (and cargo-nextest: cargo install cargo-nextest --locked).

set windows-shell := ["cmd.exe", "/c"]

default:
    @just --list

# fmt, lints, the Rust tests, the tool tests, the slice list; what stands is not run again
quick *flags:
    python tools/check/check.py quick {{flags}}

# every suite, the Linux and sandbox families too; in a worktree of a commit
full *flags:
    python tools/check/check.py full {{flags}}

# one step, crate folder, suite folder or list, or one program by name
check target *flags:
    python tools/check/check.py {{target}} {{flags}}

# each step: standing, to run, or failed, with its last run
status:
    python tools/check/check.py status

# per day: the runs, the steps run and standing, the wall time; the steps that took the most
history days="14":
    python tools/check/check.py history --days {{days}}

# a layer again, what stands too, without the pass cache
force layer="quick":
    python tools/check/check.py {{layer}} --force
