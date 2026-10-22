$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x23_key_shared)
' key$ in DIM SHARED, used in a SUB.
DIM SHARED key$
key$ = "m"
s
SYSTEM
SUB s
    PRINT key$
END SUB
