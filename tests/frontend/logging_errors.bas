' TEST: check-fail
$CONSOLE:ONLY
' The log statements' errors (m4-handlers-logging-checking task 4.1, design D2; the old compiler's in
' verification\v50_x05, x06, x15, x17-x19): a number, no message, two, _LOGMINLEVEL with parentheses or stored
' into, and a log statement used as a function.
_LOGINFO 5
_LOGINFO
_LOGINFO "a", "b"
PRINT _LOGMINLEVEL()
_LOGMINLEVEL = 3
x = _LOGINFO("a")
SYSTEM
