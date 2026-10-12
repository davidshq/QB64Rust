$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s AS STRING, t AS STRING, fx AS STRING * 8, n AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM sg AS SINGLE, fl AS _FLOAT, o AS _OFFSET, uo AS _UNSIGNED _OFFSET
lab:
DATA 1, "two", 3.5
READ fx
END
