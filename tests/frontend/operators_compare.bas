' TEST: typed
$CONSOLE:ONLY
' numeric-semantics: comparisons (m2-control-flow-slice D4). LONG -1/0; two floats compare at the narrower believed
' type, so a SINGLE literal (held as a double) is narrowed to SINGLE; otherwise C's conversions; strings call the
' runtime; integer constants fold
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT
PRINT i = l; l < q; i <> 3
PRINT s = 2.1; 2.1 = s; s >= d; d <= f
PRINT l > s; q = d
PRINT "a" < "b"; a$ = b$
PRINT 3 > 2; 2 > 3; 1 = 1
END
