$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x30_key_common_shared)
' key$ in COMMON SHARED.
COMMON SHARED key$
key$ = "c"
s
SYSTEM
SUB s
    PRINT key$
END SUB
