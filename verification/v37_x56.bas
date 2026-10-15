$CONSOLE:ONLY
' DECLARE LIBRARY, one form: (0) to a BYVAL _OFFSET that is a void* of a header (is it written NULL?).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_isnull& (BYVAL p AS _OFFSET)
END DECLARE
PRINT h_isnull&((0))
SYSTEM
