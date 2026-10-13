' TEST: ir
' The drawing statements in the IR (m3-window-output task 4.1, design D2; each call measured from the old
' compiler's C++ and compared by tests\callsite): one built-in operation each. A coordinate, a radius and an angle
' are values of the argument's own type (the runtime takes a float; nothing is converted); a colour and a style
' are LONG values, the colours of CIRCLE and PAINT _UNSIGNED LONG values; STEP, SCREEN, B and BF are choices. The
' fill of PAINT is a number or a string. VIEW with points is the first entry of the name VIEW; a coordinate of
' VIEW is a LONG value. DRAW and PCOPY have no template and are also read after CALL
DIM l AS LONG, i AS INTEGER, q AS _INTEGER64, sg AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM s AS STRING, t AS STRING, fx AS STRING * 4
PSET (1, 2)
PSET (1, 2), 3
PSET STEP(1, 2)
PSET STEP (l, l), l
PSET (i, q), q
PSET (sg, d), d
PSET (f, 1.5), 4294967295
PRESET (1, 2)
PRESET STEP(l, l), l
LINE (1, 2)-(3, 4)
LINE (1, 2)-(3, 4), 5
LINE (1, 2)-(3, 4), 5, B
LINE (1, 2)-(3, 4), 5, BF
LINE (1, 2)-(3, 4), , B
LINE (1, 2)-(3, 4), 5, b, 6
LINE (1, 2)-(3, 4), 5, , 6
LINE (1, 2)-(3, 4), , , 6
LINE (1, 2)-(3, 4),
LINE (1, 2)-(3, 4), 5,
LINE -(3, 4)
LINE -(3, 4), 5
LINE -STEP(3, 4), , B
LINE STEP(1, 2)-(3, 4)
LINE STEP (1, 2) - STEP (3, 4), 5, BF, 6
LINE (i, q)-(sg, d), d, B, q
LINE (f, 1.5)-(u, l), u
CIRCLE (1, 2), 3
CIRCLE (1, 2), 3, 4
CIRCLE (1, 2), 3, , 5
CIRCLE (1, 2), 3, , , 6
CIRCLE (1, 2), 3, , , , 7
CIRCLE STEP(1, 2), 3, 4, 5, 6, 7
CIRCLE (1, 2), 3,
CIRCLE (i, q), sg, d, d, f, 1.5
CIRCLE (l, l), l, q
CIRCLE (l, l), l, 4294967295
PAINT (1, 2)
PAINT (1, 2), 3
PAINT (1, 2), 3, 4
PAINT (1, 2), , 4
PAINT (1, 2), s
PAINT (1, 2), s, 4
PAINT (1, 2), "abc"
PAINT (1, 2), s + t, 4
PAINT (1, 2), fx
PAINT (1, 2), 3, 4, s
PAINT (1, 2), s, 4, t
PAINT (1, 2), , , s
PAINT STEP(1, 2), 3
PAINT (1, 2),
PAINT (sg, d), d, q
PAINT (i, q), 4294967295, u
DRAW s
DRAW "U10R10"
DRAW s + t
DRAW fx
CALL DRAW(s)
VIEW
VIEW (1, 2)-(3, 4)
VIEW SCREEN (1, 2)-(3, 4)
VIEW (1, 2)-(3, 4), 5
VIEW (1, 2)-(3, 4), , 6
VIEW SCREEN (1, 2)-(3, 4), 5, 6
VIEW (1, 2)-(3, 4),
VIEW (i, q)-(sg, d), d, 1.5
WINDOW
WINDOW (1, 2)-(3, 4)
WINDOW SCREEN (1, 2)-(3, 4)
WINDOW (i, q)-(sg, d)
WINDOW (f, 1.5)-(u, -1)
PALETTE
PALETTE 1, 63
PALETTE q, d
PALETTE l, 4294967295
PCOPY 1, 2
PCOPY q, d
CALL PCOPY(l, l)
SYSTEM
