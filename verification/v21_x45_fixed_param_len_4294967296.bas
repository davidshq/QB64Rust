$CONSOLE:ONLY
' Verification (m2-numeric-types, task 4.3): does the old compiler accept this? (fixed_param_len_4294967296: 0 in
' 32 bits)
q "ab"
SYSTEM
SUB q (t AS STRING * 4294967296)
    PRINT t
END SUB
