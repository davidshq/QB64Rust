$CONSOLE:ONLY
' Verification (SOMEDAY.md, Other ideas, the 40-character limit on names): a type name of 41 characters.
TYPE abcdefghijabcdefghijabcdefghijabcdefghijk
    k AS LONG
END TYPE
DIM t AS abcdefghijabcdefghijabcdefghijabcdefghijk
PRINT LEN(t)
SYSTEM
