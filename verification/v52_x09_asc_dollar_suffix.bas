$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x09_asc_dollar_suffix)
' ASC written with a suffix as the statement.
a$ = "ab"
ASC$(a$) = 65
PRINT a$
SYSTEM
