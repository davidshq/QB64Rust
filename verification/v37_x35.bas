$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a FUNCTION without a type suffix (default type).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l (BYVAL v AS LONG)
END DECLARE
PRINT h_l(2)
SYSTEM
