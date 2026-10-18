$CONSOLE:ONLY
' Verification (m3-constant-evaluation, before its proposal): the unsigned lane, the clamp of an unsuffixed literal beyond
' _INTEGER64, a suffixed constant inside a CONST, _PI and ROOT
CONST a = 18446744073709551615~&&
CONST b = a + 1
CONST u = a - 1
CONST big~&& = 18446744073709551615
CONST plain = 18446744073709551615
CONST c% = 4 + 4
CONST d = c% * 2
CONST p = _PI
CONST p2 = _PI(2)
CONST r = 20 ROOT 3
PRINT a; b; u
PRINT big~&&; plain
PRINT d
PRINT _TOSTR$(p); " "; _TOSTR$(p2); " "; _TOSTR$(r)
SYSTEM
