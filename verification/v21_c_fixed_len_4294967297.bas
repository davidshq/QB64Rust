$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.3): the length limit of a fixed-length string (STRING * 4294967297).
ON ERROR GOTO h
DIM f AS STRING * 4294967297
PRINT LEN(f)
f = "ab"
PRINT LEN(f); ASC(f, 2); ASC(f, 3)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
