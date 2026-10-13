' Runtime errors of the screen statements and functions under a handler: the number of each.
DIM n AS INTEGER, d AS DOUBLE, l AS LONG
ON ERROR GOTO handler
n = 1: SCREEN 3
n = 2: PSET (1, 1)
n = 3: LINE (1, 1)-(2, 2)
n = 4: CIRCLE (5, 5), 2
n = 5: PAINT (1, 1)
n = 6: DRAW "R5"
n = 7: LOCATE 26, 1
n = 8: LOCATE 1, 81
n = 9: VIEW PRINT 20 TO 30
n = 10: VIEW (1, 1)-(5, 5)
n = 11: WINDOW (0, 0)-(1, 1)
n = 12: d = POINT(1, 1)
n = 13: l = SCREEN(26, 1)
n = 14: PCOPY 0, 9
n = 15: COLOR 32
n = 16: WIDTH -1
n = 17: PALETTE -1, 0
n = 18: CLS 9
n = 19: SCREEN 0, , -1
n = 20: LOCATE 0
PRINT "done";
SYSTEM
handler:
PRINT "error"; ERR; "at"; n
RESUME NEXT
