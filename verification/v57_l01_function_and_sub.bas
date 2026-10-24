$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a library FUNCTION h_l& and a program SUB h_l, both called.
DECLARE LIBRARY "v57_lib/v57h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
h_l
PRINT h_l&(2)
SYSTEM
SUB h_l
    PRINT "program SUB h_l"
END SUB
