$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a block without a header whose last name is a folder only, which finds no file
' (the old compiler looks for the declarations in the last name's file).
DECLARE LIBRARY "./v37c", "v37_lib/"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
PRINT c_add&(1, 2)
SYSTEM
