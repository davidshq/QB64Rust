$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
TYPE t
    n AS LONG
    w AS STRING * 6
    d AS DOUBLE
END TYPE
DIM s AS STRING, t AS STRING, fx AS STRING * 8, n AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64, sg AS SINGLE
DIM fl AS _FLOAT, u AS _UNSIGNED LONG, b AS _BYTE, ub AS _UNSIGNED _BYTE, uq AS _UNSIGNED _INTEGER64
DIM ui AS _UNSIGNED INTEGER, o AS _OFFSET
DIM r AS t, a(1 TO 4) AS LONG, fa(2) AS STRING * 4, ra(2) AS t, dy(n) AS INTEGER, a2(2, 3) AS INTEGER
DIM sa(3) AS STRING
GET (1, 2)-(3, 4), ra(q)
END
