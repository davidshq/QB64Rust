' TEST: cpp
' The slot kinds of the screen statements in the C++ (m3-window-output task 3.1, design D2): a colour typed
' _UNSIGNED LONG is narrowed to uint32 (an integer as it is, a float through qbr); a coordinate goes as it is; the
' fill of PAINT gets the old compiler's cast, (uint32) for a number or a fill left out and (qbs*) for a string,
' which picks one of the runtime's two entries; POINT takes its arguments as they are and is a double; SCREEN as
' a function is a uint32
DIM q AS _INTEGER64, d AS DOUBLE, s AS STRING, u AS _UNSIGNED LONG
CLS , q
CLS , d
CLS , 4294967295
CIRCLE (q, d), d, d
PAINT (q, d)
PAINT (q, d), q
PAINT (q, d), d, d
PAINT (q, d), s
PAINT (q, d), s + "x", q, s
d = POINT(q, d)
d = POINT(q)
u = SCREEN(q, d)
PRINT SCREEN(1, 1); POINT(1, 1)
SYSTEM
