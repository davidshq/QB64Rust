$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x61_call_dot)
CALL a . b(3)
SYSTEM
SUB a.b (n)
    PRINT n
END SUB
