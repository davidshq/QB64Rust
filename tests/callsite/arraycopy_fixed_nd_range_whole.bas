$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM s(1 TO 5) AS STRING * 4, d(0 TO 9) AS STRING * 4
DIM m(1 TO 2, 0 TO 2) AS STRING * 4, n(1 TO 2, 0 TO 2) AS STRING * 4, big(0 TO 4, 0 TO 5) AS STRING * 4
_ARRAYCOPY big(1 TO 2, 2 TO 3) TO n()
END
