$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): STRING * n with n a DIM SHARED variable.
DIM SHARED n AS INTEGER
DIM s AS STRING * n
PRINT LEN(s)
PRINT "ok"
SYSTEM
