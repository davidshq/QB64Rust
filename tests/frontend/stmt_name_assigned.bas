' TEST: check-fail
$CONSOLE:ONLY
' A built-in statement's name, parentheses and `=`: an assignment to the old compiler, which rejects it ("Name
' already in use (CLS)"; measured for every supported statement, verification\v27_x51-x57, m3-window-output), also
' with more arguments after it and for a statement without a template. With something between the `)` and the
' `=` the line is the statement with a comparison as its argument, and compiles (the last four lines). WIDTH is
' the one name the old compiler takes as an array there: not supported yet
DIM t AS STRING
CLS (1) = 5
CLS (1) = 5, 2
LOCATE (1) = 2, 3
COLOR (1) = 1, 2
SLEEP (1) = 1
SCREEN (12) = 12
LINE (1, 2) = 5
PALETTE (1) = 1, 2
VIEW (1) = 2
SEEK (1) = 2, 3
RANDOMIZE (1) = 1
_DEST (1) = 1
KILL (t) = t
DRAW (t) = t
PCOPY (1) = 1, 2
_LIMIT (60) = 60
WIDTH (80) = 80
CLS (1) + 2 = 5
CLS ((1) = 5)
CLS (1) <> 5
LOCATE (1) * 2 = 2
SYSTEM
