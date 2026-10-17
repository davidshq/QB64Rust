$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM t AS LONG
ON TIMER(t, 0.5) GOSUB h
END
h:
RETURN
