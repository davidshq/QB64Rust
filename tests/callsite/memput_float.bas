$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
TYPE t
    a AS INTEGER
    v(0 TO 3) AS LONG
END TYPE
DIM m AS _MEM, n AS _MEM, o AS _OFFSET, k AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64, sg AS SINGLE
DIM by AS _BYTE, ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM uo AS _UNSIGNED _OFFSET, fl AS _FLOAT, s AS STRING, fx AS STRING * 6, u AS t
DIM b(10) AS INTEGER, fa(4) AS STRING * 3, ua(3) AS t, z(2) AS _MEM, b2(3, 4) AS LONG, da(5) AS DOUBLE
REDIM dy(5) AS LONG
_MEMPUT m, o, fl
END
