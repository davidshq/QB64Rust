$CONSOLE:ONLY
' Verification (m2-numeric-types, task 4.3): does the old compiler accept this? (unsigned_fixed_param)
q "ab"
SYSTEM
SUB q (t AS _UNSIGNED STRING * 3)
    PRINT t
END SUB
