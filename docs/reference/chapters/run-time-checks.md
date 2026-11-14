# Run-time checks

By default a program checks every array index and every memory access, and services errors and events after each
statement. `$CHECKING:OFF` turns that off for the lines below it.

The pages of this area: [$CHECKING](../entries/meta-checking.md).

## What $CHECKING:OFF removes

Below `$CHECKING:OFF`, indexes are not checked against the array's bounds, and `_MEMGET`, `_MEMPUT` and `_MEMFILL` do
not check their block.
<!-- src: language/run-time-checks "Indexes without checks"; language/run-time-checks "Memory statements without checks" -->

An error raised there stays pending, and an event waits, until a checked statement runs.
<!-- src: language/run-time-checks "Statements without checks"; tests\corpus\slice\s153_handlers_log_checking -->

## What it leaves alone

It changes no value of an expression, no condition of a block, `_ARRAYCOPY`'s own checks, `ON ERROR` or `RESUME`.
<!-- src: language/run-time-checks "What CHECKING leaves alone" -->

It holds for the lines after it in the order of the source text, through procedures and included files, until
`$CHECKING:ON`.
<!-- src: language/run-time-checks "The CHECKING metacommand" -->
