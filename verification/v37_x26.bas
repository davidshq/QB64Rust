$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a STRING * n parameter.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_len& (s AS STRING * 4)
END DECLARE
DIM f AS STRING * 4: f = "ab"
PRINT h_len&(f)
SYSTEM
