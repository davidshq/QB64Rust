$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x29_base_redim)
' base$ as an array in REDIM.
REDIM base$(3)
base$(3) = "b"
PRINT base$(3)
SYSTEM
