$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): variable-length STRING members: pending-error rule, copy, local.
TYPE ws
    n AS LONG
    t AS STRING
END TYPE
DIM w AS ws
DIM a(2) AS ws
DIM k AS LONG
k = -1
ON ERROR GOTO h
w.t = "a": w.t = CHR$(k)
PRINT "w.t = CHR$(-1): ["; w.t; "]"
a(1).t = "one": a(2).t = a(1).t + "!"
PRINT "a(2).t: ["; a(2).t; "]"; LEN(a(2).t)
a(0).t = "zero"
a(9).t = "nine"
PRINT "after a(9).t = nine: ["; a(0).t; "]"
addbang w.t
PRINT "addbang w.t: ["; w.t; "]"
loc
loc
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT

SUB addbang (s AS STRING)
    s = s + "!"
END SUB

SUB loc
    DIM v AS ws
    PRINT "local v.t: ["; v.t; "]"
    v.t = "set"
END SUB
