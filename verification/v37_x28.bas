$CONSOLE:ONLY
' DECLARE LIBRARY, one form: BYVAL before a TYPE parameter.
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_ptsum& (BYVAL p AS pt)
END DECLARE
DIM p AS pt: p.x = 1: p.y = 2
PRINT h_ptsum&(p)
SYSTEM
