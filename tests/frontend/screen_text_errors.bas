' TEST: check-fail
' The text-screen statements and the screen functions rejected (m3-window-output tasks 3.2 and 3.3; each form one
' program of verification\v27_x03-x50, where the old compiler rejects it too): a statement in a form that is none
' of its template's (too many arguments, a comma with nothing after it where the template has none, a fifth
' argument of SCREEN without its word, VIEW PRINT with one row), a string where a number is wanted; a function
' with empty parentheses, with an argument it does not take, with too few or too many, with a string for a number
DIM n AS LONG, d AS DOUBLE, t AS STRING, k AS STRING
LOCATE 1, 2, 3, 4, 5, 6
LOCATE , , , ,
LOCATE t
SCREEN 12, , , , 1
SCREEN 12, 0, 1, 2, 3
SCREEN t
CLS 1, 2, 3, 4
CLS t
COLOR t
WIDTH 80, 50, 1, 2, 3
WIDTH t
VIEW PRINT 2
VIEW PRINT 2 TO
VIEW PRINT TO 10
VIEW PRINT 2, 10
VIEW PRINT t TO 3
k = INKEY$()
k = INKEY$(1)
n = CSRLIN()
n = CSRLIN(1)
n = POS
n = POS()
n = POS(1, 2)
n = POS(t)
n = SCREEN
n = SCREEN()
n = SCREEN(1)
n = SCREEN(1, 2, 3, 4)
n = SCREEN(t, 1)
SYSTEM
