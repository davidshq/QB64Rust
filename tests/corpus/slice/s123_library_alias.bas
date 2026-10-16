$CONSOLE:ONLY
' DECLARE LIBRARY with a header (v37h.h in this program's .files folder): ALIAS as a string and as a
' name, C++ names (a template, a namespace) and an overload chosen by the C++ type of the argument
' (verification\v37_e, x60).
DECLARE LIBRARY "./v37h"
    FUNCTION maxl& ALIAS "h_max<int32_t>" (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION maxd# ALIAS "h_max<double>" (BYVAL a AS DOUBLE, BYVAL b AS DOUBLE)
    FUNCTION twice& ALIAS "hns::twice" (BYVAL v AS LONG)
    FUNCTION overl& ALIAS h_over (BYVAL v AS LONG)
    FUNCTION overd& ALIAS "h_over" (BYVAL v AS DOUBLE)
    FUNCTION addl& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE

PRINT "template:"; maxl&(3, 9); maxd#(2.5, -1)
PRINT "namespace:"; twice&(21)
x# = 1: y& = 1
PRINT "overload:"; overl&(0); overl&(1); overd&(1); overd&(1.5); overd&(x#); overd&(y&); overd&(y& + 0.5)
PRINT "alias name:"; addl&(40, 2)
SYSTEM
