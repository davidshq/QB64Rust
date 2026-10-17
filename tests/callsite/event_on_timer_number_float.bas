$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM t AS SINGLE, d AS DOUBLE
ON TIMER(t, d) GOSUB h
END
h:
RETURN
