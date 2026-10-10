$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s AS STRING, t AS STRING, fx AS STRING * 8, n AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64
OPEN "t.txt" FOR INPUT ACCESS READ LOCK WRITE AS #1
END
