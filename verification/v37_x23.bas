$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a header and a library: a function of the library.
DECLARE LIBRARY "v37_lib/v37h", "./v37c"
    FUNCTION c_half# (BYVAL v AS DOUBLE)
END DECLARE
PRINT c_half#(3)
SYSTEM
