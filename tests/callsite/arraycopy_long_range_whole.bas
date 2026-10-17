$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS LONG, d(0 TO 9) AS LONG
DIM m(1 TO 2, 0 TO 2) AS LONG, n(1 TO 2, 0 TO 2) AS LONG, big(0 TO 4, 0 TO 5) AS LONG
_ARRAYCOPY s(2 TO 4) TO d()
END
