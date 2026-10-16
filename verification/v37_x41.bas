$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a library name used as a variable.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
h_l& = 3
PRINT h_l&
SYSTEM
