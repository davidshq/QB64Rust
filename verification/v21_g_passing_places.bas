$CONSOLE:ONLY
' Verification (m2-numeric-types, task 8.2): an array element or a TYPE member passed to a parameter of the same
' width and the other signedness (v21_b_passing measured variables only), and the same in parentheses.
TYPE t
    sl AS LONG
    uq AS _UNSIGNED _INTEGER64
    ub AS _UNSIGNED _BYTE
END TYPE
DIM a(2) AS _UNSIGNED INTEGER, v AS t, w(1) AS t, q(1) AS _INTEGER64
a(1) = 65535: showsi a(1): PRINT " after:"; a(1)
a(1) = 65535: showsi (a(1)): PRINT " after:"; a(1)
v.sl = -1: showul v.sl: PRINT " after:"; v.sl
v.uq = 3: showso v.uq: PRINT " after:"; v.uq
w(1).ub = 200: showsb w(1).ub: PRINT " after:"; w(1).ub
q(0) = -1: showuo q(0): PRINT " after:"; q(0)
SYSTEM

SUB showsb (x AS _BYTE)
PRINT "_BYTE param:"; x;: x = -2
END SUB
SUB showsi (x AS INTEGER)
PRINT "INTEGER param:"; x;: x = -3
END SUB
SUB showul (x AS _UNSIGNED LONG)
PRINT "_UNSIGNED LONG param:"; x;: x = 5
END SUB
SUB showso (x AS _OFFSET)
PRINT "_OFFSET param:"; x;: x = -8
END SUB
SUB showuo (x AS _UNSIGNED _OFFSET)
PRINT "_UNSIGNED _OFFSET param:"; x;: x = 7
END SUB
