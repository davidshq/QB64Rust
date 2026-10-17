$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS LONG, d(0 TO 9) AS LONG, sg AS SINGLE, db AS DOUBLE
_ARRAYCOPY s(sg TO db) TO d(sg)
END
