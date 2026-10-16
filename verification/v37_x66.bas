$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a STRING * 4 member followed by a LONG of 65 (bytes 41 00 00 00): does C see a NUL
' after the member's bytes, or read on into the next member?
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_len& (s AS STRING)
END DECLARE
TYPE t
    f AS STRING * 4
    g AS LONG
END TYPE
DIM v AS t
v.f = "abcd": v.g = 65
PRINT h_len&(v.f)
SYSTEM
