$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): STRING * n with n a name never assigned.
DIM s AS STRING * nn
PRINT LEN(s)
PRINT "ok"
SYSTEM
