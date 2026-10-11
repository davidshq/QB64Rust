' TEST: check-fail
$CONSOLE:ONLY
' `_BIT` arrays (m2-defxxx-common-arrays task 4.2; measured, verification\v24_x82, v24_b_bit_arrays): they are
' declared by AS or by suffix, signed or unsigned, of one bit or n; SWAP of two elements is "Cannot SWAP bit-length
' variables", LEN of one "Variable/element cannot be _BIT aligned", and console INPUT into one "INPUT cannot handle
' _BIT array elements" (v24_x253). READ and INPUT # store into one (v24_x251, x252).
DIM f(7) AS _BIT
DIM g(3) AS _UNSIGNED _BIT * 9
DIM h`5(2)
f(1) = g(2) + h`5(0)
SWAP f(0), f(1)
PRINT LEN(g(1))
INPUT g(1)
INPUT "n"; n, f(2)
READ f(1), g(2)
INPUT #1, f(1), g(2)
DATA 1, 2
SYSTEM
