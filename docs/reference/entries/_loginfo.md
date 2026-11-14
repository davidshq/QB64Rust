# _LOGINFO

`_LOGINFO msg$` adds an entry to the runtime's log at the information level. `_LOGTRACE`, `_LOGWARN` and `_LOGERROR`
add one at their level.

## Syntax

```text
_LOGTRACE msg$
_LOGINFO msg$
_LOGWARN msg$
_LOGERROR msg$
```

## Behavior

Which entries are written, and where, is the runtime's, set by the `QB64PE_LOG_*` environment variables. The log does
not go to the program's output.
<!-- src: language/logging "Log statements"; tests\corpus\slice\s153_handlers_log_checking; tests\reference\examples\_loginfo_1 -->

An entry names the file, the procedure and the line of the statement, as QB64pe names them: the procedure's C name, or
`Main QB64 Code`.
<!-- src: language/logging "The place of a log entry" -->

## Errors

- An argument that is not a string, no argument or two, and use as a function are compile errors. A string in
  parentheses is accepted.
  <!-- src: language/logging "The log statements' errors" -->

## Differences

From QB64pe: when the folder the compiler runs from has a name that is not ASCII, the file name in an entry is in the
system's code page there, so the bytes differ from machine to machine. QB64Rust writes it in UTF-8
(`DIVERGENCES-QB64PE.md` D-033).
<!-- src: D-033 -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_loginfo_1 -->
```qb64
_LOGTRACE "a trace entry"
_LOGINFO "an information entry"
_LOGWARN "a warning"
_LOGERROR "an error entry"
PRINT "The log does not go to the program's output"
END
```

Output:

```text
The log does not go to the program's output

Press any key to continue
```
<!-- end generated -->

## See also

[`_LOGMINLEVEL`](_logminlevel.md), [`_ECHO`](_echo.md).
