$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): the pending-error rule. Under RESUME NEXT, does a store or a call
' made after a raising operation in the same statement still happen, and with which placeholder value?
DIM k AS LONG
k = -1
ON ERROR GOTO h
x = 5: x = ASC("")
PRINT "x = ASC(empty):"; x
x = 5: x = 10 + ASC("")
PRINT "x = 10 + ASC(empty):"; x
y& = 5: y& = LEN(CHR$(k))
PRINT "y& = LEN(CHR$(-1)):"; y&
z# = 5: z# = SQR(-1)
PRINT "z# = SQR(-1):"; z#
w! = 5: w! = (-8) ^ (1 / 3)
PRINT "w! = (-8) ^ (1/3):"; w!
s$ = "a": s$ = CHR$(k)
PRINT "s$ = CHR$(-1): ["; s$; "]"; LEN(s$)
s$ = "a": s$ = "<" + CHR$(k) + ">"
PRINT "s$ = < + CHR$(-1) + >: ["; s$; "]"
s$ = "a": s$ = MID$("abc", 0, 1)
PRINT "s$ = MID$(abc, 0, 1): ["; s$; "]"
show ASC("")
PRINT "after show ASC(empty)"
showstr CHR$(k)
PRINT "after showstr CHR$(-1)"
n = 1: n = n + 1: n = n + ASC(""): n = n + 100
PRINT "n after four statements on a line:"; n
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT

SUB show (v AS SINGLE)
    PRINT "in show"; v
END SUB

SUB showstr (t AS STRING)
    PRINT "in showstr ["; t; "]"
END SUB
