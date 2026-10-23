$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x57_writefile_number)
' _WRITEFILE with a number as the data.
_WRITEFILE "v52_x57.tmp", 5
PRINT LEN(_READFILE$("v52_x57.tmp"))
KILL "v52_x57.tmp"
SYSTEM
