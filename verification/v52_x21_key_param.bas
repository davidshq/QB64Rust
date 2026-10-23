$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x21_key_param)
' key$ as a parameter.
s "k"
SYSTEM
SUB s (key$)
    PRINT key$
END SUB
