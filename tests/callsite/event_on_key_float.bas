$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM k AS DOUBLE
ON KEY(k) GOSUB h
END
h:
RETURN
