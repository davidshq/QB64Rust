$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM a(-5 TO -1) AS LONG, b(-9 TO 0) AS LONG
_ARRAYCOPY a(-4 TO -2) TO b(-9)
END
