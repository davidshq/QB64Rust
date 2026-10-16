$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a header without a guard that only declares, named by two blocks (x16's defines).
DECLARE LIBRARY "./v37c"
    FUNCTION c_ticks& ()
END DECLARE
DECLARE LIBRARY "v37_lib/v37q"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
DECLARE LIBRARY "v37_lib/v37q"
    FUNCTION c_half# (BYVAL v AS DOUBLE)
END DECLARE
PRINT c_add&(1, 2); c_half#(5); c_ticks&
SYSTEM
