$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 10, an element of a STRING * 3 array passed to a library's
' char*: the length strlen sees, and a change in place (upper case up to the first NUL) with the index read from a
' FUNCTION that counts its calls.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_len& (s AS STRING)
    SUB h_upper (s AS STRING)
END DECLARE
DIM SHARED calls AS LONG
DIM e(3) AS STRING * 3
e(0) = "abc": e(1) = "def": e(2) = "ghi": e(3) = "jkl"
PRINT "1."; h_len&(e(1))
h_upper e(ix)
PRINT "2. "; e(0); " "; e(1); " "; e(2); " "; e(3); calls
SYSTEM

FUNCTION ix&
    calls = calls + 1
    ix& = 1
END FUNCTION
