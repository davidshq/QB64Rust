$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x03_asc_string_position)
' A string as the position.
a$ = "ab"
ASC(a$, "2") = 65
PRINT a$
SYSTEM
