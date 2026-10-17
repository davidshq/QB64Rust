$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM x AS LONG
ON TIMER(1.5) s x * 2
END
SUB s (n AS LONG)
END SUB
