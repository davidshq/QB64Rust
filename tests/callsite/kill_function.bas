$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s AS STRING, t AS STRING, fx AS STRING * 8, a(3) AS STRING, n AS LONG
KILL LEFT$(s, n) + CHR$(65)
END
