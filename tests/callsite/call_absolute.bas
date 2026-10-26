$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM v AS INTEGER, s AS STRING, o AS _UNSIGNED INTEGER
CALL ABSOLUTE(v, s, 2.5#, o)
END
