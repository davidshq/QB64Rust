$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS _OFFSET, d(0 TO 9) AS _OFFSET
_ARRAYCOPY s(1 TO 2) TO d(0)
END
