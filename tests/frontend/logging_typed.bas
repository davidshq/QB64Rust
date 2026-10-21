' TEST: typed
$CONSOLE:ONLY
' The log statements (m4-handlers-logging-checking task 4.1, design D2): each level with a string message (an
' expression, in parentheses, in any letter case), and _LOGMINLEVEL, a LONG (verification\v50_b_log, x16).
_LOGTRACE "trace"
_LOGINFO "info " + STR$(_LOGMINLEVEL)
_LOGWARN ("warn")
_logerror "error"
PRINT _LOGMINLEVEL
x& = _LOGMINLEVEL
SYSTEM
