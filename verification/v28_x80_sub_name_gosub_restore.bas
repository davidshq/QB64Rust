$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): GOSUB, RESTORE and ON ERROR GOTO as the jump that makes the name a label.
DIM v AS INTEGER
GOSUB one
RESTORE two
READ v
PRINT "read"; v
ON ERROR GOTO three
ERROR 5
PRINT "back"
SYSTEM
one:
PRINT "label one"
RETURN
DATA 1
two:
DATA 2
three:
PRINT "label three"
RESUME NEXT
SUB one
    PRINT "sub one"
END SUB
SUB two
END SUB
SUB three
END SUB
