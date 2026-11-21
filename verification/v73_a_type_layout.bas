$CONSOLE:ONLY
' Verification (wasm-pointer-width, design D8): the sizes that follow a pointer's, each compared with W, the
' pointer's bytes on the platform (8, or 4 where $IF 32BIT is true), so that one recording holds for both. An
' _OFFSET, a TYPE of a LONG, an _OFFSET and a STRING * 2, an array of it, and its members read back.
$IF 32BIT THEN
    CONST W = 4
$ELSE
    CONST W = 8
$END IF
TYPE rec
    a AS LONG
    p AS _OFFSET
    s AS STRING * 2
END TYPE
DIM p AS _OFFSET, u AS _UNSIGNED _OFFSET, v AS rec, items(3) AS rec
PRINT LEN(p) = W; LEN(u) = W; LEN(v) = 6 + W; LEN(items()) = 4 * (6 + W)
v.a = 11: v.p = 22: v.s = "ab"
items(2) = v
items(3).a = 5: items(3).p = 7: items(3).s = "cd"
PRINT items(2).a; items(2).p; items(2).s; items(3).a; items(3).p; items(3).s
PRINT items(1).a; items(1).p; items(0).a
SWAP items(2), items(3)
PRINT items(2).a; items(2).p; items(2).s; items(3).a; items(3).p; items(3).s
p = 100: u = 200
SWAP p, u
PRINT p; u
SYSTEM
