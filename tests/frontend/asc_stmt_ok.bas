' TEST: ir
$CONSOLE:ONLY
' The ASC statement in the IR (m4-qb64pe-sources task 3.1; measured in verification\v52_d_asc): a variable, an
' element, a member of each kind, a fixed-length string, a parameter, a name not declared, with and without the
' position; the position and the value stored as LONG (a float rounded, an _INTEGER64 cut)
TYPE t
    v AS STRING
    f AS STRING * 3
END TYPE
DIM a AS STRING, fx AS STRING * 4, arr(3) AS STRING, farr(2) AS STRING * 2, u AS t, ua(2) AS t
DIM d AS DOUBLE, q AS _INTEGER64
ASC(a) = 65
ASC(a, 2) = 66
ASC(fx, 4) = 33
ASC(arr(2), 1) = d
ASC(farr(1)) = q
ASC(u.v, 2.6) = 87
ASC(u.f) = 75
ASC(ua(1).v, q) = 82
ASC(nd$) = 65
setfirst a
SUB setfirst (s$)
    ASC(s$) = 67
END SUB
