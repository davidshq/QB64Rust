$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x69_declare_library_typo)
DECLARE LIBRARY
    FUNCTION labs& (BYVAL n AS _Bytef)
END DECLARE
PRINT "ok"
SYSTEM
