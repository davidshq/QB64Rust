$CONSOLE:ONLY
' DECLARE LIBRARY without a name (the C runtime the program's C++ already reaches) and DECLARE DYNAMIC
' LIBRARY "kernel32" (found in System32, loaded when the program starts; verification\v37_a, v37_g).
DECLARE LIBRARY
    FUNCTION isdigit& (BYVAL c AS LONG)
    FUNCTION toupper& (BYVAL c AS LONG)
    FUNCTION labs& ALIAS abs (BYVAL v AS LONG)
    FUNCTION strlen~%& (s AS STRING)
    FUNCTION atoi& (s AS STRING)
    FUNCTION fabs# (BYVAL v AS DOUBLE)
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION lstrlenA& (s AS STRING)
    FUNCTION GetCurrentProcessId~&
    SUB SetLastError (BYVAL e AS _UNSIGNED LONG)
    FUNCTION GetLastError~&
END DECLARE

PRINT isdigit&(ASC("2")) <> 0; isdigit&(ASC("x"))
PRINT CHR$(toupper&(ASC("q"))); labs&(-17); fabs#(-2.5)
PRINT strlen~%&("four"); atoi&("123abc")
PRINT "kernel32:"; lstrlenA&("seven!!"); GetCurrentProcessId~& <> 0
' Read before PRINT, whose output could set the last error.
SetLastError 1234: e~& = GetLastError~&: PRINT "last error:"; e~&
SYSTEM
