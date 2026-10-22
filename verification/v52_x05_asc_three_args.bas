$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x05_asc_three_args)
' Three arguments.
a$ = "ab"
ASC(a$, 1, 2) = 65
PRINT a$
SYSTEM
