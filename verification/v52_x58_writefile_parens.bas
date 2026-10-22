$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x58_writefile_parens)
' _WRITEFILE with its arguments in parentheses.
_WRITEFILE ("v52_x58.tmp", "a")
PRINT LEN(_READFILE$("v52_x58.tmp"))
KILL "v52_x58.tmp"
SYSTEM
