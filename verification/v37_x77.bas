$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a FUNCTION of a block whose header declares the C function, and a SUB of the same C
' function in a block of the library file without a header, which writes a prototype for the SUB.
DECLARE LIBRARY "v37_lib/v37q"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
DECLARE LIBRARY "./v37c"
    SUB c_add (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
c_add 1, 2
PRINT c_add&(3, 4)
SYSTEM
