$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a library FUNCTION h_l (no suffix) and a program SUB h_l.
DECLARE LIBRARY "v57_lib/v57h"
    FUNCTION h_l (BYVAL v AS LONG)
END DECLARE
h_l
PRINT "ok"
SYSTEM
SUB h_l
    PRINT "program SUB h_l"
END SUB
