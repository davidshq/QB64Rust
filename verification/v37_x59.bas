$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a CONST 0 to a BYVAL _OFFSET that is a void* of a header (is it written NULL?).
CONST z = 0
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_isnull& (BYVAL p AS _OFFSET)
END DECLARE
PRINT h_isnull&(z)
SYSTEM
