' TEST: ir
' _MOUSEMOVE of m3-images-fonts-mouse in the IR (design D1 and D3: compiled, run by no test, its call compared by
' tests\callsite): DOUBLE slots, each number converted to DOUBLE; after CALL the same operation
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
_MOUSEMOVE 10, 20
_MOUSEMOVE i, l
_MOUSEMOVE q, s
_MOUSEMOVE d, f
_MOUSEMOVE u, u
CALL _MOUSEMOVE(10, 20)
