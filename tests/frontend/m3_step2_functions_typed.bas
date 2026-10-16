' TEST: typed
$CONSOLE:ONLY
' The functions of m3-get-put-bload with their held and believed types (design D2, D3; verification\v28_x01,
' v28_d_varptr_index): TAB and SPC give a string and are plain items of PRINT; PEEK a LONG; VARPTR and VARSEG an
' _INTEGER64 (against the table's LONG); _RGB32 with one to four arguments and _RGBA32 an _UNSIGNED LONG;
' _FULLSCREEN a LONG; DATE$ and TIME$ strings
DIM a(1 TO 10) AS INTEGER, i AS INTEGER, l AS LONG, s AS SINGLE, q AS _INTEGER64, u AS _UNSIGNED LONG, t AS STRING
PRINT TAB(5); "a"; SPC(s); TAB(q)
PRINT #1, TAB(i); "a"
t = TAB(5) + SPC(i)
l = LEN(TAB(l))
l = PEEK(i) + PEEK(s)
q = VARPTR(a(i)) + 1
s = VARSEG(a(1)) / 4
i = VARPTR(a(2))
u = _RGB32(1) + _RGB32(1, 2) + _RGB32(i, l, s) + _RGB32(1, 2, 3, 4)
u = _RGBA32(i, l, s, q)
l = _FULLSCREEN
t = DATE$ + TIME$
PRINT VARPTR(a(2)); _RGB32(1, 2, 3); PEEK(1); _FULLSCREEN
