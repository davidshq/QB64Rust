$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS STRING, d(0 TO 9) AS STRING
DIM m(1 TO 2, 0 TO 2) AS STRING, n(1 TO 2, 0 TO 2) AS STRING, big(0 TO 4, 0 TO 5) AS STRING
_ARRAYCOPY s(2 TO 4) TO d()
END
