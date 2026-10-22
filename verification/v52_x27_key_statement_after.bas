$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x27_key_statement_after)
' key$ as a variable, then the statement KEY.
key$ = "2"
KEY 1, "x"
PRINT key$
SYSTEM
