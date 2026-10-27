' TEST: check-ok
$CONSOLE:ONLY
' VARPTR, VARSEG and VARPTR$ of a TYPE variable that holds a STRING, of its members (the STRING member's pointer
' too) and of an element of an array of it (m4-left-marked task 4.1, design D3; verification\v56_x08, x26, x27,
' v60_d1-d4): the variable lives in conventional memory, its strings made (QB64PE-DIVERGENCES.md D-037).
TYPE inner
    s AS STRING
    v AS LONG
END TYPE
TYPE ts
    n AS STRING
    v AS LONG
    i AS inner
END TYPE
DIM rs AS ts, rsa(3) AS ts, s AS STRING
PRINT VARPTR(rs.v); VARSEG(rs); VARPTR(rs.n); VARPTR(rs.i.s); VARPTR(rs.i.v)
s = VARPTR$(rs.v)
PRINT VARPTR(rsa(1).v); VARSEG(rsa(2).i.v)
rs.n = "abc": rsa(1).i.s = "d"
PRINT rs.n; rsa(1).i.s
SYSTEM
