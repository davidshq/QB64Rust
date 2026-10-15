$CONSOLE:ONLY
' DECLARE LIBRARY, one form: DEFLNG before a parameter without a type given 1.5.
DEFLNG V
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_d# (BYVAL v)
END DECLARE
PRINT h_d#(1.5)
SYSTEM
