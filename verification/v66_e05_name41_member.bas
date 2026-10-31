$CONSOLE:ONLY
' Verification (SOMEDAY.md, Other ideas, the 40-character limit on names): a member name of 41 characters.
TYPE t1
    abcdefghijabcdefghijabcdefghijabcdefghijk AS LONG
END TYPE
DIM t AS t1
PRINT LEN(t)
SYSTEM
