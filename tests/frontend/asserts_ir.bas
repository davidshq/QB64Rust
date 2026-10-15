' TEST: ir
' _ASSERT as a call of its row (m3-asserts-debug-zlib design D3), with and without the message, after CALL; the
' program's settings say asserts (design D1), from $ASSERTS:CONSOLE, which also gives a console.
$ASSERTS:CONSOLE
DIM k AS LONG, d AS DOUBLE, s AS STRING
_ASSERT k
_ASSERT d, s
CALL _ASSERT(k = 1, "m")
SYSTEM
