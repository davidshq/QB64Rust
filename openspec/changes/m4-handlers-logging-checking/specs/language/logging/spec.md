# Spec Delta

## Purpose

The program's own entries in the runtime's log: the statements `_LOGTRACE`, `_LOGINFO`, `_LOGWARN` and
`_LOGERROR`, and the function `_LOGMINLEVEL`, as the old compiler writes them (`verification\v50_*`).

## ADDED Requirements

### Requirement: Log statements
`_LOGTRACE msg$`, `_LOGINFO msg$`, `_LOGWARN msg$` and `_LOGERROR msg$` SHALL add an entry to the runtime's log
at their level, with the message. The entry SHALL name the file, the procedure and the line of the statement as
the old compiler names them: the source file as given to the compiler, or the included file; the procedure's
name, or `Main QB64 Code`; the line within that file. Which entries are written and where SHALL be the runtime's,
set by `QB64PE_LOG_*`.

#### Scenario: Four levels at the default level
- **WHEN** `logging/commands` is run as the runner runs it
- **THEN** it prints nothing, and its log holds the error, warning and information entries with lines 3, 4 and 5
  and not the trace entry

#### Scenario: In a SUB and an included file
- **WHEN** `_LOGINFO "x"` stands in a `SUB s` and in a file included by the main module
- **THEN** the log's entries name the procedure and the file and line as the old compiler's build does (`v50`)

### Requirement: The log statements' errors
A log statement whose argument is not a string, or that has no argument, SHALL be a compile error, as in the old
compiler.

#### Scenario: A number
- **WHEN** `_LOGINFO 5` is compiled
- **THEN** it is a compile error, as in the old compiler (`v50`)

### Requirement: The minimum level
`_LOGMINLEVEL` SHALL return the runtime's minimum level of the log as a LONG, as the old compiler's build does.

#### Scenario: Default level
- **WHEN** `PRINT _LOGMINLEVEL` runs with `QB64PE_LOG_LEVEL` not set
- **THEN** it prints what the old compiler's build prints (`v50`)
