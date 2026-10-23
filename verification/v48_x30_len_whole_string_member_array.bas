$CONSOLE:ONLY
' Verification (m4-names-arrays, review): LEN of a whole member array of variable-length strings (LEN of a whole
' array of them is an error, v48_x01).
TYPE t
    s(3) AS STRING
END TYPE
DIM x AS t
PRINT LEN(x.s())
SYSTEM
