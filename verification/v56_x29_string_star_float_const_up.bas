$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): STRING * n with n a CONST of 2.7 and of 3.5 (how a float length is rounded).
CONST n = 2.7, m = 3.5, k = 4.5
DIM s AS STRING * n, t AS STRING * m, u AS STRING * k
PRINT LEN(s); LEN(t); LEN(u)
PRINT "ok"
SYSTEM
