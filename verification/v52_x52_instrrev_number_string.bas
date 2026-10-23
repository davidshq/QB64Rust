$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x52_instrrev_number_string)
' _INSTRREV with a number as the string.
PRINT _INSTRREV(1, 2, "a")
SYSTEM
