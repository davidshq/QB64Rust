$CONSOLE
' Verification (m4-statements-metacommands, task 1.1): the function _SCREENHIDE after the statement _SCREENHIDE.
_SCREENHIDE
_DEST _CONSOLE
PRINT "hidden:"; _SCREENHIDE
_SCREENSHOW
PRINT "after show:"; _SCREENHIDE
SYSTEM
