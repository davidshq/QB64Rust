$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a declaration repeated in a second block.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
PRINT h_l&(1)
SYSTEM
