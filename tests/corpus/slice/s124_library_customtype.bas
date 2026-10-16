$CONSOLE:ONLY
' DECLARE CUSTOMTYPE LIBRARY with a header (v37h.h in this program's .files folder): the function is
' reached through a pointer of the declared type, so the declaration's types may differ from the C
' function's (verification\v37_h).
DECLARE CUSTOMTYPE LIBRARY "v37h"
    FUNCTION h_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION addll&& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB h_pl (v AS LONG)
END DECLARE

x& = 4: h_pl x&
PRINT "customtype:"; h_add&(1, 2); addll&&(3, 4); x&
SYSTEM
