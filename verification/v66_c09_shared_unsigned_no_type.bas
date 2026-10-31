$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-upstream-rejections row on names that are no type): SHARED name AS _UNSIGNED and a name that is no type.
DIM n AS LONG
s
SYSTEM

SUB s
    SHARED n AS _UNSIGNED nosuch
    PRINT n
END SUB
