$CONSOLE
$SCREENHIDE
' Verification (step 11): an untrapped error with a hidden window and the console as destination (run with
' QB64PE_NOPROMPT=y, the runner's default).
_DEST _CONSOLE
PRINT "before"
ERROR 5
PRINT "after"
SYSTEM
