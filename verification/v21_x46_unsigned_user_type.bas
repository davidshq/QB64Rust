$CONSOLE:ONLY
' Verification (m2-numeric-types, task 4.1): does the old compiler accept this? (unsigned_user_type)
TYPE pair
    a AS LONG
END TYPE
DIM p AS _UNSIGNED pair
PRINT LEN(p)
SYSTEM
