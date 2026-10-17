$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS DOUBLE, d(0 TO 9) AS DOUBLE, i AS INTEGER, q AS _INTEGER64, u AS _UNSIGNED LONG
_ARRAYCOPY s(i TO q) TO d(u)
END
