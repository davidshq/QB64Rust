$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s AS STRING, t AS STRING, fx AS STRING * 8, n AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64, sg AS SINGLE
DIM fl AS _FLOAT, u AS _UNSIGNED LONG
PRINT HEX$(_RED32(u)); _RED32(u) + 1; _ALPHA32(u)
END
