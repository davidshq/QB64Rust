$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x68_whole_then_two)
PRINT "ok"
SYSTEM
PRINT LEN(x())
x(1, 2) = 1
