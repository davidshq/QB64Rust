$CONSOLE:ONLY
' DECLARE LIBRARY with a header: what a parameter without BYVAL receives for each kind of argument.
DECLARE LIBRARY "v37_lib/v37h"
    SUB h_pl (v AS LONG)
    SUB h_ps (v AS SINGLE)
    FUNCTION h_isnull& (v AS LONG)
    FUNCTION h_isnull_o& ALIAS h_isnull (BYVAL v AS _OFFSET)
    FUNCTION h_peekl& (BYVAL p AS _OFFSET)
    FUNCTION h_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB h_tick
    FUNCTION h_ticks&
END DECLARE
TYPE t
    a AS LONG
    b AS LONG
END TYPE

DIM x AS LONG, a(3) AS LONG, v AS t, k AS INTEGER, sg AS SINGLE
x = 10
h_pl x: PRINT "variable:"; x
h_pl (x): PRINT "parenthesised:"; x
h_pl x + 0: PRINT "expression:"; x
a(2) = 20: h_pl a(2): PRINT "element:"; a(2)
v.b = 30: h_pl v.b: PRINT "member:"; v.b
h_pl 5: PRINT "literal: done"
k = 40: h_pl k: PRINT "other type:"; k
sg = 1.5: h_ps sg: PRINT "single:"; sg
CALL h_pl(x): PRINT "call:"; x
PRINT "null literal 0:"; h_isnull&(0); " null 1:"; h_isnull&(1); " null var:"; h_isnull&(x)
PRINT "byval offset 0:"; h_isnull_o&(0)
x = 1234: PRINT "peek:"; h_peekl&(_OFFSET(x))
PRINT "add:"; h_add&(2, 3)
h_tick: h_tick: CALL h_tick
PRINT "ticks:"; h_ticks&
SYSTEM
