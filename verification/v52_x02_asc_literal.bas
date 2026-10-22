$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x02_asc_literal)
' ASC of a string literal as the place.
ASC("ab") = 65
PRINT "done"
SYSTEM
