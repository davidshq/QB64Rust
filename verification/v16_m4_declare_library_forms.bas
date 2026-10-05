$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): DECLARE LIBRARY with BYVAL, ALIAS, suffixes and a SUB.
DECLARE LIBRARY
    FUNCTION toupper& (BYVAL c AS LONG)
    FUNCTION up2& ALIAS "toupper" (BYVAL c AS LONG)
    FUNCTION up3~& ALIAS toupper (BYVAL c AS _UNSIGNED LONG)
    ' a comment
    SUB srand (BYVAL seed AS _UNSIGNED LONG)
END DECLARE
PRINT toupper&(97); up2&(98); up3~&(99)
SYSTEM
