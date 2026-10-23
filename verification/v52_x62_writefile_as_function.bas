$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x62_writefile_as_function)
' _WRITEFILE used as a function.
x = _WRITEFILE("v52_x62.tmp", "a")
SYSTEM
