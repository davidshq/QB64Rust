$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a FUNCTION without a suffix whose C function returns a double (default type).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_d (BYVAL v AS DOUBLE)
END DECLARE
PRINT h_d(1.5)
SYSTEM
