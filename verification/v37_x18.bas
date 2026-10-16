$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a .a beside the program named without ./.
DECLARE LIBRARY "v37c"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
PRINT c_add&(1, 2)
SYSTEM
