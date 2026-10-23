$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): STRING * n with n a CONST of a float with no fraction.
CONST n = 3#
DIM s AS STRING * n
PRINT LEN(s)
PRINT "ok"
SYSTEM
