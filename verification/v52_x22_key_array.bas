$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x22_key_array)
' key$ as an array.
DIM key$(2)
key$(1) = "a"
PRINT key$(1); UBOUND(key$)
SYSTEM
