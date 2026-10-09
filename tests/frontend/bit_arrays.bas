' TEST: check-fail
$CONSOLE:ONLY
' `_BIT` arrays are "not supported yet" (m2-numeric-types task 6.1; the old compiler packs their elements bit by bit,
' `verification\v21_b_bit_scopes`), signed or unsigned, of one bit or n, by `AS` or by suffix.
DIM f(7) AS _BIT
DIM g(3) AS _UNSIGNED _BIT * 9
DIM h`5(2)
SYSTEM
