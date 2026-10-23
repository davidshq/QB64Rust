$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x65_shared_dot)
DIM SHARED a . b AS LONG
a.b = 5
s
SYSTEM
SUB s
    PRINT a.b
END SUB
