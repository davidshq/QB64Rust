$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): RETURN label, then a RETURN with nothing pending, then a normal GOSUB.
ON ERROR GOTO h
GOSUB r1
PRINT "not reached"
r1back:
PRINT "at r1back"
RETURN
PRINT "after RETURN"
GOSUB g
PRINT "end"
SYSTEM
r1:
PRINT "in r1"
RETURN r1back
g:
PRINT "in g"
RETURN
h:
PRINT "handler"; ERR
RESUME NEXT
