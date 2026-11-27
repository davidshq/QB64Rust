$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a library SUB and a library FUNCTION of one name, and a program FUNCTION of it too.
DECLARE LIBRARY "v37_lib/v37h"
    SUB h_l (BYVAL v AS LONG)
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
h_l 1
PRINT h_l&(2)
SYSTEM
FUNCTION h_l& (v AS LONG)
    h_l& = v * 10
END FUNCTION
