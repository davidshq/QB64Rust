$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a DEFxxx applies from where it stands in the file, not from
' where the program runs: a variable met before and after it, DIM without a type before it, DIM AS before it,
' a statement skipped by GOTO and one inside an IF block that does not run. LEN of a variable shows its type:
' 2 INTEGER, 4 LONG or SINGLE, 8 DOUBLE.
a = 1.5
DEFINT A-Z
b = 1.5
PRINT "a = 1.5 before DEFINT A-Z, b = 1.5 after: a, b"; a; b
PRINT "a!, a%:"; a!; a%
a = 7
PRINT "after a = 7: a!, a%:"; a!; a%
DEFSNG A-Z
DIM c
c = 2.5
DIM d AS LONG
d = 100000
DEFINT A-Z
PRINT "DIM c, c = 2.5, then DEFINT: c, c!, c%, LEN(c)"; c; c!; c%; LEN(c)
PRINT "DIM d AS LONG, then DEFINT: d, LEN(d)"; d; LEN(d)
DEFSNG A-Z
GOTO skip
DEFLNG G
skip:
g = 2.5: PRINT "DEFLNG G skipped by GOTO: g, LEN(g)"; g; LEN(g)
IF 0 THEN
    DEFDBL H
END IF
h = 1# / 3: PRINT "DEFDBL H in an IF block that does not run: h"; h
SYSTEM
