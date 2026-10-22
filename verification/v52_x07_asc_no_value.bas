$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x07_asc_no_value)
' No = and value.
a$ = "ab"
ASC(a$)
PRINT a$
SYSTEM
