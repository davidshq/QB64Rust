$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS _UNSIGNED _BYTE, d(0 TO 9) AS _UNSIGNED _BYTE
_ARRAYCOPY s() TO d(3)
END
