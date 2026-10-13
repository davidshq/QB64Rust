$CONSOLE
$SCREENHIDE
' Verification (step 11): a trapped error with the console as destination.
ON ERROR GOTO handler
_DEST _CONSOLE
PRINT "before"
ERROR 5
PRINT "after"
x = 1 / 0
PRINT "end"
SYSTEM

handler:
PRINT "error"; ERR; "line"; _ERRORLINE
RESUME NEXT
