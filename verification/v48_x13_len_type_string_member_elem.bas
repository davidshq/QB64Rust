$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): LEN of one element of an array of a TYPE with a STRING member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(3) AS ts
a(1).n = "abc"
PRINT LEN(a(1))
SYSTEM
