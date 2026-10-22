$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x53_instrrev_string_start)
' _INSTRREV with a string as the start.
PRINT _INSTRREV("1", "a", "b")
SYSTEM
