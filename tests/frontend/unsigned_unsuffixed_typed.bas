' TEST: typed
$CONSOLE:ONLY
' An integer literal without a suffix above 9223372036854775807 (m4-line-numbers-whole-arrays, design D3): the
' old compiler adds `~&&`, so it is held and believed `_UNSIGNED _INTEGER64` as `18446744073709551615~&&` is.
' Printed, stored into `~&&`, `&&` and DOUBLE, and in an operation (`verification\v54_b_unsigned_literals`).
DIM u AS _UNSIGNED _INTEGER64, i AS _INTEGER64, d AS DOUBLE
PRINT 18446744073709551615; 9223372036854775808
u = 18446744073709551615: i = 18446744073709551615: d = 18446744073709551615
PRINT 18446744073709551614 + 1; 9223372036854775808 / 2
SYSTEM
