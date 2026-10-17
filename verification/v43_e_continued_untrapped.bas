$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): the line of the message of an error nothing traps, raised
' by a statement before the underscore of a line continued with one.
PRINT "before"
ERROR 5: x = 1 + _
2
PRINT "after"
SYSTEM
