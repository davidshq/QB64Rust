$CONSOLE:ONLY
' DECLARE LIBRARY, one form: 0 to a BYVAL LONG of a header function that is not overloaded.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
PRINT h_l&(0)
SYSTEM
