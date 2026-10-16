$CONSOLE:ONLY
' DECLARE CUSTOMTYPE LIBRARY: the function is reached through a pointer of the declared type, so the
' declaration's types may differ from the C function's (here a header function, and a .a without one).
DECLARE CUSTOMTYPE LIBRARY "v37_lib/v37h"
    FUNCTION h_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION addll&& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB h_pl (v AS LONG)
END DECLARE
DECLARE CUSTOMTYPE LIBRARY "./v37c"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE

x& = 4: h_pl x&
PRINT "customtype:"; h_add&(1, 2); addll&&(3, 4); x&; c_add&(5, 6)
SYSTEM
