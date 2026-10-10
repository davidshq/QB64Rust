$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (90_read_parameter)
DIM n AS LONG
p n
PRINT n
SYSTEM
DATA 7
SUB p (x AS LONG)
    READ x
END SUB
