' TEST: cpp
$CONSOLE:ONLY
' The log statements in C++ (m4-handlers-logging-checking task 4.2, design D2): libqb's macro with the file (as
' given here: --dump cpp puts no folder before it), "Main QB64 Code" or the procedure's C name (a FUNCTION with a
' suffix, a SUB with a dot), the line in that file, and the message; an included file's own name and line.
_LOGINFO "main"
_LOGTRACE "a" + STR$(_LOGMINLEVEL)
s
PRINT f$
a.b
'$INCLUDE:'inc/log.bi'
SYSTEM

SUB s
    _LOGERROR "in s"
END SUB

FUNCTION f$
    _LOGWARN "in f"
    f$ = "f"
END FUNCTION

SUB a.b
    _LOGINFO "in a.b"
END SUB
