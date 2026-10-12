' TEST: cpp
$CONSOLE:ONLY
' The C++ of `_BIT` stores (m2-numeric-types design D5, task 6.1): an unsigned `_BIT * n` is masked to its low n
' bits, a signed one sign-extended from bit n-1, at 1 bit, 32 bits (4 bytes of storage) and above (8 bytes, so a
' wide one never shares bytes with its neighbour, DIVERGENCES.md D-009); in each storage class (DIM SHARED, STATIC,
' a local); from a float (rounded by `qbr`); a `_BIT` variable passed to a LONG parameter as a copy. Same output as
' qb64pe.exe (each wide `_BIT` after a `_BIT * 32` pad, so the old compiler's overlap hits only the pads).
DIM SHARED sh AS _BIT * 4
DIM b1 AS _BIT, u1 AS _UNSIGNED _BIT, b32 AS _BIT * 32, u32 AS _UNSIGNED _BIT * 32
DIM pad1 AS _BIT * 32, b33 AS _BIT * 33
DIM pad2 AS _BIT * 32, u64 AS _UNSIGNED _BIT * 64
b1 = 3: u1 = 3: b32 = 4294967295: u32 = -1: b33 = -1: u64 = 2.5
sh = 9
PRINT b1; u1; b32; u32; b33; u64; sh
st
showl b1
SYSTEM

SUB st
STATIC s AS _BIT * 4
DIM l AS _UNSIGNED _BIT * 4
s = s + 9: l = 17: sh = sh + 1
PRINT s; l; sh
END SUB

SUB showl (x AS LONG)
x = 5
END SUB
