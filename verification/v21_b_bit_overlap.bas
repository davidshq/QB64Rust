$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.2): which variable a _BIT * n above 32 overwrites (D-009). Its C type
' is 8 bytes but 4 are reserved; is the victim the variable DIMmed before or after it, and of any type?
DIM l1 AS LONG, w AS _BIT * 40, l2 AS LONG
l1 = 5: l2 = 6
w = -1
PRINT "LONG before, LONG after:"; l1; l2; w
DIM s1 AS SINGLE, w2 AS _BIT * 40
s1 = 1.5
w2 = 0
PRINT "SINGLE before:"; s1
DIM i1 AS INTEGER, i2 AS INTEGER, w3 AS _BIT * 33
i1 = 1: i2 = 2
w3 = -1
PRINT "two INTEGERs before:"; i1; i2
DIM u3 AS _UNSIGNED _BIT * 3, w4 AS _BIT * 40
u3 = 2
w4 = 2
PRINT "_UNSIGNED _BIT * 3 before:"; u3
DIM q1 AS _INTEGER64, w5 AS _BIT * 40
q1 = 7
w5 = -1
PRINT "_INTEGER64 before:"; q1
DIM w6 AS _BIT * 40, a1 AS _BIT * 8
a1 = 9
w6 = -1
PRINT "_BIT * 8 after:"; a1
SYSTEM
