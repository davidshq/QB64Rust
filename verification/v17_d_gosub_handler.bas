$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOSUB from an error handler, then RESUME NEXT.
ON ERROR GOTO h
ERROR 7
PRINT "after ERROR 7"
SYSTEM
hsub:
PRINT "in hsub"
RETURN
h:
PRINT "handler"; ERR
GOSUB hsub
PRINT "handler after GOSUB"; ERR
RESUME NEXT
