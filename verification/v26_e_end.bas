$CONSOLE
$SCREENHIDE
' Verification (step 11): END in a program with a hidden window and the console as destination. Does it wait for
' a key, and what does it print?
_DEST _CONSOLE
PRINT "before END"
END
