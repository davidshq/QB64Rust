$CONSOLE:ONLY
' DECLARE LIBRARY, one form: DEFLNG before a FUNCTION without a suffix whose C function returns a double:
' 1.5 gives 2.5, which rounding and truncation both make 2; 1.7 gives 2.7, 3 if rounded, 2 if truncated.
DEFLNG H
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_d (BYVAL v AS DOUBLE)
END DECLARE
PRINT h_d(1.5); h_d(1.7)
SYSTEM
