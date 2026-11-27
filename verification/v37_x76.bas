$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a SUB and a FUNCTION of one name in a CUSTOMTYPE block of a library with no header,
' which declares the C function once per declaration and calls each through a pointer of its own.
DECLARE CUSTOMTYPE LIBRARY "./v37c"
    SUB c_add (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
c_add 1, 2
PRINT c_add&(3, 4)
SYSTEM
