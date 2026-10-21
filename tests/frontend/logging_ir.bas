' TEST: ir
$CONSOLE:ONLY
' The log statements in the IR (m4-handlers-logging-checking task 4.2, design D2): one Log operation each, at its
' level, with the message; the file, procedure and line are the emitter's.
_LOGTRACE "trace"
_LOGINFO "info " + STR$(_LOGMINLEVEL)
_LOGWARN "warn"
_LOGERROR "error"
SYSTEM
