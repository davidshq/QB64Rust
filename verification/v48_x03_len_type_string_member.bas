$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): LEN of a whole array of a TYPE with a STRING member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(3) AS ts
PRINT LEN(a())
SYSTEM
