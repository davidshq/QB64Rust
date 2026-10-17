$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM c3(1 TO 2, 1 TO 2, 1 TO 2) AS INTEGER, e3(1 TO 2, 1 TO 2, 1 TO 2) AS INTEGER
_ARRAYCOPY c3(2 TO 2, 1 TO 2, 2 TO 2) TO e3(1, 1, 1)
END
