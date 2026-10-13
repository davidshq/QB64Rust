$CONSOLE
$SCREENHIDE
' Verification (step 11): INPUT and LINE INPUT after _DEST _CONSOLE and _SOURCE _CONSOLE, standard input from a
' file.
_DEST _CONSOLE
_SOURCE _CONSOLE
INPUT "n"; a
PRINT "a ="; a
LINE INPUT "l? "; l$
PRINT "l$ = ["; l$; "]"
SYSTEM
