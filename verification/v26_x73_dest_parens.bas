$CONSOLE
$SCREENHIDE
' Verification (step 11, group 3): the handle in parentheses, and the functions in an expression.
_DEST (_CONSOLE)
PRINT _DEST + _SOURCE; _CONSOLE * 2; -_DEST
_SOURCE (_DEST)
PRINT _SOURCE
SYSTEM
