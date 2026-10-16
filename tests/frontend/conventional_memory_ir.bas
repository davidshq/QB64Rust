' TEST: ir
$CONSOLE:ONLY
' Conventional memory (m3-get-put-bload design D2 to D4; verification\v28_b_bload, v28_d_varptr_index,
' v28_e_cmem_arrays): DEF SEG in both forms, POKE, BLOAD with and without an offset, BSAVE, each slot converted as
' a LONG; VARPTR and VARSEG as expressions of their own, typed _INTEGER64, the index of VARSEG dropped. An array
' named in one of them is in conventional memory wherever the use stands: after its DIM (a), in a procedure (g),
' through COMMON (cm, dynamic), a dynamic one (dy); b and n are not named and stay where they were
COMMON SHARED cm() AS INTEGER
DIM SHARED cm(1 TO 20) AS INTEGER
DIM SHARED g(1 TO 6) AS INTEGER
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER, m(1 TO 3, 1 TO 4) AS LONG, n AS INTEGER, s AS SINGLE
DIM l AS LONG, q AS _INTEGER64, t AS STRING
n = 8
DIM dy(1 TO n) AS LONG
a(3) = 7
DEF SEG = VARSEG(a(1))
DEF SEG = s
DEF SEG
POKE VARPTR(a(n)), 65
POKE s, q
l = PEEK(VARPTR(m(2, 3)) + 1)
BLOAD "f.bin", VARPTR(a(1))
BLOAD t
BSAVE t + ".bin", VARPTR(a(1)), 20
q = VARSEG(dy(1)) + VARSEG(cm(n + 1))
l = VARPTR(dy(3))
b(1) = n
CALL fill
_FULLSCREEN
_FULLSCREEN _SQUAREPIXELS, _SMOOTH
_FULLSCREEN OFF
SYSTEM

SUB fill
    DEF SEG = VARSEG(g(1))
    POKE VARPTR(g(6)), 9
    DEF SEG
END SUB
