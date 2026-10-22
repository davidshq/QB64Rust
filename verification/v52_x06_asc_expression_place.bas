$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x06_asc_expression_place)
' An expression as the place.
a$ = "ab": b$ = "c"
ASC(a$ + b$) = 65
PRINT a$
SYSTEM
