$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
ON ERROR GOTO _NEWHANDLER h
END
h:
RESUME NEXT
