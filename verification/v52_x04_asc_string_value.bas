$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x04_asc_string_value)
' A string as the value.
a$ = "ab"
ASC(a$) = "A"
PRINT a$
SYSTEM
