$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a _BIT parameter and result.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l` (BYVAL v AS _BIT)
END DECLARE
PRINT h_l`(0)
SYSTEM
