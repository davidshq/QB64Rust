$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a parameter without a type (default type).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_s! (BYVAL v)
END DECLARE
PRINT h_s!(2)
SYSTEM
