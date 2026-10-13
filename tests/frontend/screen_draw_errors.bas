' TEST: check-fail
' The drawing and pacing statements and POINT rejected (m3-window-output tasks 4.1 and 5.1; each form one program
' of verification\v27_x03-x50, where the old compiler rejects it too): a statement in a form that is none of its
' template's (LINE without its second point, a point without parentheses, a comma with nothing after it where the
' template has none, a box word that is neither B nor BF, too many arguments, a word alone), a string where a
' number is wanted and a number where a string is; POINT bare, with three arguments, with a string
DIM n AS LONG, d AS DOUBLE, t AS STRING, k AS STRING
LINE (1, 2), 3
LINE (1, 2)
LINE
LINE (1, 2)-(3, 4), 5, B,
LINE (1, 2)-(3, 4), 5, X
LINE (1, 2)-(3, 4), 5, B, 6, 7
LINE (1, 2)-(3, 4), t
PSET
PSET 1, 2
PSET (1, 2),
PSET (1, 2), 3, 4
PSET (1, 2, 3)
PSET (t, 2)
PSET (1, 2), t
CIRCLE (1, 2)
CIRCLE (1, 2),
CIRCLE (1, 2), 3, 4, 5, 6, 7, 8
CIRCLE (1, 2), t
CIRCLE (1, 2), 3, t
PAINT
PAINT (1, 2), 3, t
PAINT (1, 2), 3, 4, 5
PAINT (1, 2), 3, 4, t, 5
PAINT (t, 2)
DRAW
DRAW 1
DRAW t, k
VIEW (1, 2)
VIEW SCREEN
VIEW (1, 2)-(3, 4), 5,
VIEW (1, 2)-(3, 4), 5, 6, 7
VIEW STEP(1, 2)-(3, 4)
VIEW (t, 2)-(3, 4)
WINDOW SCREEN
WINDOW (1, 2)
WINDOW (1, 2)-(3, 4), 5
WINDOW (t, 2)-(3, 4)
PALETTE 1
PALETTE 1, 2, 3
PALETTE t, 2
PCOPY 1
PCOPY
PCOPY t, 1
_LIMIT
_LIMIT 1, 2
_LIMIT t
_DELAY
_DELAY t
SLEEP 1, 2
SLEEP t
_DISPLAY 1
d = POINT
d = POINT()
d = POINT(1, 2, 3)
d = POINT(t)
d = POINT(1, t)
SYSTEM
