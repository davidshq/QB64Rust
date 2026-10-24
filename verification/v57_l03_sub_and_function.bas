$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a library SUB h_v and a program FUNCTION h_v&.
DECLARE LIBRARY "v57_lib/v57h"
    SUB h_v (BYVAL v AS LONG)
END DECLARE
h_v 3
PRINT h_v&(2)
SYSTEM
FUNCTION h_v& (v AS LONG)
    PRINT "program FUNCTION h_v&"
    h_v& = v * 10
END FUNCTION
