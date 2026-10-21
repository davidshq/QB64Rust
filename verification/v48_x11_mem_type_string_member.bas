$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): _MEM of a whole array of a TYPE with a STRING member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(3) AS ts
DIM m AS _MEM
m = _MEM(a())
PRINT m.SIZE
SYSTEM
