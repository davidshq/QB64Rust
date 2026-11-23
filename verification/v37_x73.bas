$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a SUB and a FUNCTION of one name, each in a block of its own.
DECLARE LIBRARY "v37_lib/v37h"
    SUB h_l (BYVAL v AS LONG)
END DECLARE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
h_l 1
PRINT h_l&(2)
SYSTEM
