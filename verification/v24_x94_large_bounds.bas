$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM with bounds from -2^62 to 2^62, 1 byte each.
ON ERROR GOTO h
DIM lo AS _INTEGER64, hi AS _INTEGER64
lo = -4611686018427387904: hi = 4611686018427387904
PRINT "DIM a(lo TO hi):";
DIM a(lo TO hi) AS _BYTE
PRINT " then UBOUND(a):";
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
