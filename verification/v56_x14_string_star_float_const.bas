$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): STRING * n with n a CONST of a float.
CONST n = 2.5
DIM s AS STRING * n
PRINT LEN(s)
PRINT "ok"
SYSTEM
