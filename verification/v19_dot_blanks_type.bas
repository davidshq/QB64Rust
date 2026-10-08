$CONSOLE:ONLY
' Blanks on both sides of a member dot after a plain TYPE variable (`a . s`, found in upstream ERASE lines).
TYPE t
    s AS LONG
END TYPE
DIM a AS t
a . s = 5
PRINT a . s
PRINT a.s
SYSTEM
