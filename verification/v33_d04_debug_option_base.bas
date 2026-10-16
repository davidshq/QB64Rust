$CONSOLE:ONLY
$DEBUG
' Verification (m3-asserts-debug-zlib, task 1.1): $DEBUG with OPTION BASE 1 after it (vwatch.bi dimensions arrays before the first line).
OPTION BASE 1
DIM a(3)
PRINT LBOUND(a)
SYSTEM
