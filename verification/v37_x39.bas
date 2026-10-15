$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a STRING result of NULL.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_nullstr$
END DECLARE
PRINT "["; h_nullstr$; "]"
SYSTEM
