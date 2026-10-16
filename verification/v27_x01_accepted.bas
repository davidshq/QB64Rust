$CONSOLE:ONLY
' Verification (m3-window-output, task 1.3): forms of the screen statements and functions the old compiler
' accepts, as code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, t AS STRING, k AS STRING
PRINT "ok"
SYSTEM
SCREEN
SCREEN 12,
SCREEN 12, , , ,
SCREEN 12, , , , _MANUALDISPLAY 1
SCREEN , , , , _MANUALDISPLAY 1
CLS ,
CLS 1,
CLS , , 3
COLOR
COLOR 1,
COLOR 1, 2, 3, 4
COLOR , , , 4
LOCATE
LOCATE 1,
LOCATE , , , , 4
WIDTH
WIDTH , 50
WIDTH 80, 50, 1, 2
WIDTH #1, 80
WIDTH #1
WIDTH #1, 80, 50
WIDTH LPRINT 80
WIDTH LPRINT
VIEW PRINT
VIEW
VIEW (1, 2)-(3, 4),
WINDOW
PSET STEP (1, 1)
PRESET STEP(1, 2), 3
LINE -(5, 5)
LINE -(3, 4), , , 6
LINE -STEP(3, 4), , B
LINE STEP(1, 1)-STEP(2, 2)
LINE STEP (1, 2)-STEP (3, 4)
LINE (1, 2) - (3, 4)
LINE (1, 2)-(3, 4),
LINE (1, 2)-(3, 4), 5,
LINE (1, 2)-(3, 4), 5, b
LINE (1, 2)-(3, 4), 5, bf
LINE (1, 2)-(3, 4), 5, , 6
CIRCLE (1, 2), 3,
CIRCLE (1, 2), 3, , , , 7
PAINT (1, 2),
PAINT (1, 2), 3,
PAINT (1, 2), t
PAINT (1, 2), t, 4, k
PAINT (1, 2), , , t
PALETTE
SLEEP
k = INKEY$
l = CSRLIN
l = POS(0)
d = POINT(0)
d = POINT(1, 2)
l = SCREEN(1, 2)
l = SCREEN(1, 2, 1)
l = INKEY
k = CSRLIN$
