$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x34_key_shared_in_sub)
' key$ in SHARED in a SUB.
key$ = "h"
s
SYSTEM
SUB s
    SHARED key$
    PRINT key$
END SUB
