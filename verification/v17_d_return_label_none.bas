$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): RETURN label with no GOSUB pending (handler resumes next), then a GOSUB.
ON ERROR GOTO h
RETURN lab
PRINT "after RETURN lab"
lab:
PRINT "at lab"
GOSUB g
PRINT "end"
SYSTEM
g:
PRINT "in g"
RETURN
h:
PRINT "handler"; ERR
RESUME NEXT
