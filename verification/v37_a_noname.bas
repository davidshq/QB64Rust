$CONSOLE:ONLY
' DECLARE LIBRARY without a name: functions the program's C++ already reaches (the C runtime).
DECLARE LIBRARY
    FUNCTION isdigit& (BYVAL c AS LONG)
    FUNCTION toupper& (BYVAL c AS LONG)
    FUNCTION labs& ALIAS abs (BYVAL v AS LONG)
    FUNCTION strlen~%& (s AS STRING)
    FUNCTION atoi& (s AS STRING)
    FUNCTION fabs# (BYVAL v AS DOUBLE)
END DECLARE

PRINT isdigit&(ASC("2")) <> 0; isdigit&(ASC("x"))
PRINT CHR$(toupper&(ASC("q"))); labs&(-17); fabs#(-2.5)
PRINT strlen~%&("four"); atoi&("123abc")
SYSTEM
