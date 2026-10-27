$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 10, a STRING * 4 member followed by another passed to a
' library's char*: the length strlen sees, and a change in place (upper case up to the first NUL).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_len& (s AS STRING)
    SUB h_upper (s AS STRING)
END DECLARE
TYPE t
    a AS STRING * 4
    b AS STRING * 4
    z AS LONG
END TYPE
DIM v AS t
v.a = "abcd": v.b = "efgh": v.z = 0
PRINT "1."; h_len&(v.a)
h_upper v.a
PRINT "2. "; v.a; " "; v.b
SYSTEM
