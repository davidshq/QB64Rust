$CONSOLE:ONLY
' Verification (m2-numeric-types, task 4.3): does the old compiler accept this? (unsigned_string_param)
q "ab"
SYSTEM
SUB q (t AS _UNSIGNED STRING)
    PRINT t
END SUB
