$CONSOLE:ONLY
' Functions of the C runtime, which every program's C++ already reaches, so the block names no library.
DECLARE LIBRARY
    FUNCTION toupper& (BYVAL c AS LONG)
    FUNCTION labs& ALIAS abs (BYVAL v AS LONG)
    FUNCTION strlen~%& (s AS STRING)
    FUNCTION atoi& (s AS STRING)
END DECLARE
PRINT "toupper: "; CHR$(toupper&(ASC("q")))
PRINT "abs, by its ALIAS:"; labs&(-17)
PRINT "strlen:"; strlen~%&("four")
PRINT "atoi:"; atoi&("123abc")
END
