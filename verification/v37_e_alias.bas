$CONSOLE:ONLY
' DECLARE LIBRARY with a header: a TYPE passed as void*, ALIAS as a string and as a name, C++ names.
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
DECLARE LIBRARY "./v37_lib/v37h"
    FUNCTION h_ptsum& (p AS pt)
    SUB h_ptset (p AS pt)
    FUNCTION maxl& ALIAS "h_max<int32_t>" (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION maxd# ALIAS "h_max<double>" (BYVAL a AS DOUBLE, BYVAL b AS DOUBLE)
    FUNCTION twice& ALIAS "hns::twice" (BYVAL v AS LONG)
    FUNCTION overl& ALIAS h_over (BYVAL v AS LONG)
    FUNCTION overd& ALIAS "h_over" (BYVAL v AS DOUBLE)
    FUNCTION addl& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE

DIM p AS pt, q(2) AS pt
p.x = 3: p.y = 4
PRINT "udt:"; h_ptsum&(p)
h_ptset p: PRINT "udt set:"; p.x; p.y
q(1).x = 5: q(1).y = 6: PRINT "udt element:"; h_ptsum&(q(1))
PRINT "template:"; maxl&(3, 9); maxd#(2.5, -1)
PRINT "namespace:"; twice&(21)
x# = 1: y& = 1
PRINT "overload:"; overl&(1); overd&(1); overd&(1.5); overd&(x#); overd&(y&); overd&(y& + 0.5)
PRINT "alias name:"; addl&(40, 2)
SYSTEM
