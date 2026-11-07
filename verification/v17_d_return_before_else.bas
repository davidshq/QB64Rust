$CONSOLE:ONLY
' Verification: RETURN just before ELSE in a one-line IF, bare and with a label, in the main module and in a SUB.
' Found in the Civil War Battleset port: IF stax = 0 THEN leave = 1: RETURN ELSE CALL clrlin(23)
FOR i = 0 TO 1
    a = i
    GOSUB g
    PRINT "back from g, a ="; a
NEXT
a = 2
GOSUB r
PRINT "back from r, a ="; a
a = 1
GOSUB r
PRINT "not reached"
back:
PRINT "back at label"
s 0
s 1
SYSTEM
g:
IF a = 0 THEN PRINT "a is 0": RETURN ELSE PRINT "a is not 0"
PRINT "after the IF"
RETURN
r:
IF a = 1 THEN RETURN back ELSE PRINT "a is not 1"
RETURN
SUB s (b)
    GOSUB l
    PRINT "back in s, b ="; b
    EXIT SUB
    l:
    IF b = 0 THEN RETURN ELSE PRINT "b is not 0"
    RETURN
END SUB
