$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): the line _ERRORLINE gives a statement on a line continued
' with an underscore: before the continuation, after it, over three lines, a single-line IF, a block IF's
' condition, and the line that follows (a FUNCTION in a continued single-line IF's condition: v43_x46).
ON ERROR GOTO h
ERROR 5: x = 1 + _
2
x = 1 + _
2: ERROR 6
ERROR _
_
7
IF x = 3 THEN _
ERROR 8
IF x = _
3 THEN
    ERROR 9
END IF
ERROR 14
SYSTEM

h:
PRINT "error"; ERR; "_ERRORLINE"; _ERRORLINE
RESUME NEXT
