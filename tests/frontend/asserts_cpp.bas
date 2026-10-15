' TEST: cpp
' $ASSERTS turns the global asserts on (m3-asserts-debug-zlib design D1; the old compiler's global.txt has
' `int32 asserts=1;`, verification/v33_a01) wherever it stands, here on the last line; _ASSERT is the plain call
' of its row, the condition an INTEGER slot: an integer as it is, a float through qbr_float_to_long (design D3).
$CONSOLE:ONLY
DIM k AS LONG, d AS DOUBLE, s AS STRING
_ASSERT k
_ASSERT d, s
_ASSERT k = 1, "m"
SYSTEM
$ASSERTS
