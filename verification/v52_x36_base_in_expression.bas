$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x36_base_in_expression)
' base$ in an expression with other strings.
base$ = "b": c$ = base$ + "c" + base$
PRINT c$; LEN(base$)
SYSTEM
