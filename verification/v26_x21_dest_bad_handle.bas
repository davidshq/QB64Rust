$CONSOLE
$SCREENHIDE
' Verification (step 11): _DEST with a handle that does not exist, trapped; the destination afterwards.
ON ERROR GOTO handler
_DEST _CONSOLE
PRINT "console"
_DEST -99
PRINT "after the bad handle"
_DEST 1.6
PRINT "after 1.6"
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT
