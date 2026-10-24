$CONSOLE
$SCREENHIDE
' Verification (m4-builtins-remaining, task 2.1): WAIT on the display status port (it returns when a vertical retrace has
' happened), with an xor mask, on a port libqb does not emulate (it returns at once), and its arguments out of
' range (error 6) under a handler.
ON ERROR GOTO handler
SCREEN 13
WAIT &H3DA, 8
WAIT &H3DA, 8, 0
WAIT &H378, 1
WAIT &H378, 1, 1
WAIT 70000, 1
WAIT &H378, 70000
WAIT &H378, 1, 70000
WAIT -70000, 1
_DEST _CONSOLE
PRINT "ok"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
