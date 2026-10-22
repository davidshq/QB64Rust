$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x01_asc_number_place)
' ASC of a numeric variable as the place.
n% = 1
ASC(n%) = 65
PRINT n%
SYSTEM
