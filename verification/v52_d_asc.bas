$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): the ASC statement: the first byte and a position, positions 0, -1,
' the length and beyond it, an empty string, values beyond a byte and below 0, a value of each type, a
' fixed-length string, an array element, a member of each kind, a parameter (by reference), a name not declared,
' and the order in which the place, the position and the value are evaluated, under a handler that prints ERR.
TYPE t
    v AS STRING
    f AS STRING * 3
END TYPE
DIM SHARED trace$
ON ERROR GOTO h
a$ = "a/b": ASC(a$, 2) = 92: PRINT a$
a$ = "xyz": ASC(a$) = 65: PRINT a$
a$ = "xyz": ASC(a$, 1) = 66: PRINT a$
a$ = "xyz": ASC(a$, 3) = 67: PRINT a$
PRINT "out of range"
a$ = "ab": ASC(a$, 3) = 65: PRINT a$
a$ = "ab": ASC(a$, 0) = 65: PRINT a$
a$ = "ab": ASC(a$, -1) = 65: PRINT a$
b$ = "": ASC(b$) = 65: PRINT "["; b$; "]"
b$ = "": ASC(b$, 1) = 65: PRINT "["; b$; "]"
PRINT "values"
a$ = "a": ASC(a$) = 321: PRINT ASC(a$)
a$ = "a": ASC(a$) = -1: PRINT ASC(a$)
a$ = "a": ASC(a$) = 66.6: PRINT ASC(a$)
a$ = "a": ASC(a$) = 66.5: PRINT ASC(a$)
a$ = "a": ASC(a$) = 67.5: PRINT ASC(a$)
a$ = "a": v&& = 4294967363&&: ASC(a$) = v&&: PRINT ASC(a$)
a$ = "a": u~% = 68: ASC(a$) = u~%: PRINT ASC(a$)
a$ = "abc": ASC(a$, 2.6) = 90: PRINT a$
a$ = "abc": ASC(a$, 1.5) = 90: PRINT a$
PRINT "places"
DIM fx AS STRING * 4
fx = "wxyz": ASC(fx, 4) = 33: PRINT fx
fx = "": ASC(fx) = 34: PRINT "["; fx; "]"
DIM arr(3) AS STRING
arr(2) = "pq": ASC(arr(2), 2) = 81: PRINT arr(2)
DIM farr(3) AS STRING * 2
farr(1) = "mn": ASC(farr(1)) = 77: PRINT farr(1)
DIM u AS t
u.v = "uv": ASC(u.v, 2) = 87: PRINT u.v
u.f = "ijk": ASC(u.f, 3) = 75: PRINT u.f
DIM ua(2) AS t
ua(1).v = "rs": ASC(ua(1).v) = 82: PRINT ua(1).v
c$ = "cd": setfirst c$: PRINT c$
ASC(nd$) = 65: PRINT "["; nd$; "]"
PRINT "order"
a$ = "abc": trace$ = ""
ASC(a$, pos1) = val1: PRINT a$; " "; trace$
a$ = "abc": trace$ = ""
ASC(a$) = val1: PRINT a$; " "; trace$
a$ = "": trace$ = ""
ASC(a$, pos1) = val1: PRINT "["; a$; "] "; trace$
a$ = "abc": trace$ = ""
ASC(a$, 9) = val1: PRINT a$; " "; trace$
SYSTEM
h:
PRINT "ERR"; ERR
RESUME NEXT
SUB setfirst (s$)
    ASC(s$) = 67
END SUB
FUNCTION pos1
    trace$ = trace$ + "p"
    pos1 = 1
END FUNCTION
FUNCTION val1
    trace$ = trace$ + "v"
    val1 = 88
END FUNCTION
