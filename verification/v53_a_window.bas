$CONSOLE
$RESIZE:ON
' Verification (m4-qb64pe-builtins, task 2.1): the window and console built-ins that a test can see. _CONSOLE OFF
' and ON in a $CONSOLE program whose console is the runner's pipe (twice each, and the function _CONSOLE while
' off), _EXIT with no request to end (twice), the statement _RESIZE in each form, and the drop built-ins with
' nothing dropped: _ACCEPTFILEDROP as function before and after each form of the statement, _TOTALDROPPEDFILES,
' _DROPPEDFILE with and without $, with and without an index (1, 0, below 0), _FINISHDROP with and without CALL.
ON ERROR GOTO handler
_DEST _CONSOLE
PRINT "a"
_CONSOLE OFF
PRINT "b"
_CONSOLE OFF
PRINT "b2"; _CONSOLE
_CONSOLE ON
PRINT "c"
_CONSOLE ON
PRINT "d"
PRINT _EXIT; _EXIT
PRINT _ACCEPTFILEDROP
_ACCEPTFILEDROP
PRINT _ACCEPTFILEDROP
_ACCEPTFILEDROP OFF
PRINT _ACCEPTFILEDROP
_ACCEPTFILEDROP ON
PRINT _ACCEPTFILEDROP
PRINT _TOTALDROPPEDFILES
PRINT "["; _DROPPEDFILE$; "]["; _DROPPEDFILE; "]["; _DROPPEDFILE$(1); "]["; _DROPPEDFILE(0); "]["; _DROPPEDFILE$(-5); "]"
_FINISHDROP
CALL _FINISHDROP
PRINT _TOTALDROPPEDFILES
_RESIZE OFF
_RESIZE ON
_RESIZE ON, _STRETCH
_RESIZE , _SMOOTH
_RESIZE OFF, _SMOOTH
_RESIZE
PRINT "end"
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT
