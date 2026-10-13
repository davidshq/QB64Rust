$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s AS STRING, t AS STRING, fx AS STRING * 8, n AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64, sg AS SINGLE
DIM fl AS _FLOAT, u AS _UNSIGNED LONG
DIM a(100) AS INTEGER, b(1 TO 10, 1 TO 20) AS LONG, c(100) AS SINGLE
PUT (1, 2), b
END
