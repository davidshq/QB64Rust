# Logging

A program writes entries to the runtime's log with four statements, one per level, and reads the log's minimum level.
Where the log goes is set outside the program.

The pages of this area: [_LOGINFO](../entries/_loginfo.md), [_LOGMINLEVEL](../entries/_logminlevel.md).

## Entries and levels

`_LOGTRACE`, `_LOGINFO`, `_LOGWARN` and `_LOGERROR` each add an entry at their level, with a string message. The log
is not the program's output: it goes where the `QB64PE_LOG_*` environment variables send it, a file under the test
runner.
<!-- src: language/logging "Log statements"; tests\reference\examples\_loginfo_1 -->

An entry names the file, the procedure and the line of the statement, as QB64pe names them; a statement of the main
module names `Main QB64 Code`.
<!-- src: language/logging "The place of a log entry" -->

`_LOGMINLEVEL` gives the level below which entries are not written. It is 2 when no variable sets it.
<!-- src: language/logging "The minimum level"; tests\reference\examples\_logminlevel_1 -->

## What the compiler rejects

A log statement takes exactly one string. A number, no argument, two, or the statement used as a function, is a
compile error.
<!-- src: language/logging "The log statements' errors" -->
