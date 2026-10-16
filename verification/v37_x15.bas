$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a TYPE named before its definition.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_ptsum& (p AS pt)
END DECLARE
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
DIM p AS pt: p.x = 1: p.y = 2
PRINT h_ptsum&(p)
SYSTEM
