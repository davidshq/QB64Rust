$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): the store rule of a string array element, and initial values.
DIM s(3) AS STRING
DIM k AS LONG
k = -1
ON ERROR GOTO h
PRINT "initial: ["; s(0); "]"; LEN(s(3))
s(1) = "a": s(1) = CHR$(k)
PRINT "s(1) = CHR$(-1): ["; s(1); "]"
s(0) = "zero"
s(4) = "b"
PRINT "after s(4) = b: ["; s(0); "] ["; s(3); "]"
s(4) = CHR$(k)
PRINT "after s(4) = CHR$(-1): ["; s(0); "]"
t$ = "t": t$ = s(4)
PRINT "t$ = s(4): ["; t$; "]"
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
