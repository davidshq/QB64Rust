$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a BYVAL _OFFSET literal to a void* of a header.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_isnull& (BYVAL p AS _OFFSET)
END DECLARE
PRINT h_isnull&(5)
SYSTEM
