' TEST: ir
' The text-screen statements in the IR (m3-window-output task 3.2, design D2; each call measured from the old
' compiler's C++ and compared by tests\callsite): one built-in operation each, every argument that is left out an
' absent slot, every number a LONG value (a float rounded, a wider integer narrowed), CLS's colour an _UNSIGNED
' LONG value. SCREEN's fifth argument stands after the word _MANUALDISPLAY; WIDTH's # and LPRINT are a choice.
' VIEW PRINT is the second entry of the name VIEW. The program has no console: its Settings line says a window
DIM l AS LONG, i AS INTEGER, q AS _INTEGER64, sg AS SINGLE, d AS DOUBLE, u AS _UNSIGNED LONG
SCREEN
SCREEN 12
SCREEN l, l
SCREEN l, , l, l
SCREEN , , l, l
SCREEN 12,
SCREEN 12, , , , _MANUALDISPLAY 1
SCREEN sg
SCREEN q, d
CLS
CLS ,
CLS 2
CLS , l
CLS , , l
CLS l, l, l
CLS , 4294967295
CLS , u
CLS , q
CLS , d
CLS , 1.5
COLOR
COLOR 14
COLOR , 1
COLOR 14, 1
COLOR 14, 1, 4
COLOR , , , l
COLOR l, l, l, l
COLOR q, d
COLOR 4294967295
LOCATE
LOCATE 5
LOCATE , 10
LOCATE 5, 10
LOCATE , , 1
LOCATE 5, 10, 1, 3, 5
LOCATE , , , 3, 5
LOCATE i, q
LOCATE sg, 2.5
WIDTH
WIDTH 80
WIDTH , 50
WIDTH 80, 50
WIDTH 80, 50, 8, 16
WIDTH #1, 80
WIDTH #l
WIDTH LPRINT 80
WIDTH LPRINT
WIDTH q, d
VIEW PRINT
VIEW PRINT 2 TO 10
VIEW PRINT i TO q
VIEW PRINT sg TO 10.5
SYSTEM
