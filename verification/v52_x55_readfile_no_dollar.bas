$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x55_readfile_no_dollar)
' _READFILE without $.
PRINT _READFILE("a")
SYSTEM
