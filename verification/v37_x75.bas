$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a SUB and a FUNCTION of one name in a block of a library with no header, so a
' prototype is written for each declaration of the one C function.
DECLARE LIBRARY "./v37c"
    SUB c_add (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
c_add 1, 2
PRINT c_add&(3, 4)
SYSTEM
