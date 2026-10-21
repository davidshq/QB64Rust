$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): the four log statements and _LOGMINLEVEL (the runner
' logs to a file at the default level), in the main module, a SUB, a FUNCTION with a suffix, a SUB with a dot
' in its name, an included file and a SUB in an included file; a message made of an expression, an empty one,
' and one longer than the runtime keeps. The entries are in v50_b_log.log.txt.
PRINT "min level"; _LOGMINLEVEL
_LOGTRACE "trace main"
_LOGINFO "info main"
_LOGWARN "warn main"
_LOGERROR "error main"
_loginfo "info " + STR$(1 + 1) + LTRIM$(STR$(_LOGMINLEVEL))
_LOGINFO ""
_LOGINFO STRING$(250, "x")
s
PRINT "f$ = "; f$("y")
a.b
'$INCLUDE: 'v50_inc/log.bi'
inc_sub
PRINT "done"
SYSTEM

SUB s
    _LOGINFO "info s"
    _LOGERROR "error s"
END SUB

FUNCTION f$ (x$)
    _LOGWARN "warn f " + x$
    f$ = x$ + x$
END FUNCTION

SUB a.b
    _LOGINFO "info a.b"
END SUB

'$INCLUDE: 'v50_inc/logsub.bm'
