$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a library SUB called with parentheses and no CALL.
DECLARE LIBRARY "v37_lib/v37h"
    SUB h_pl (v AS LONG)
END DECLARE
x& = 1: h_pl (x&): PRINT x&
SYSTEM
