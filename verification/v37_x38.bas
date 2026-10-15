$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a parameter with a type suffix.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v&)
END DECLARE
PRINT h_l&(2)
SYSTEM
