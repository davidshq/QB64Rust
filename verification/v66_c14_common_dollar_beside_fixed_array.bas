$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-qb64pe-sources row on the $ name of a STRING * n array): COMMON a$() beside a STRING * 4 array a.
DIM a(3) AS STRING * 4
COMMON a$()
a$(1) = "xy"
PRINT LEN(a(1)); "["; a$(1); "]"
SYSTEM
