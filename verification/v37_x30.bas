$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a function the header does not declare.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_none& (BYVAL v AS LONG)
END DECLARE
PRINT h_none&(1)
SYSTEM
