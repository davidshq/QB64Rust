$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a name in another letter case than the C name.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION H_L& (BYVAL v AS LONG)
END DECLARE
PRINT h_l&(1)
SYSTEM
