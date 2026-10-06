$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB leaves by EXIT SUB with its own GOSUB pending; main then GOSUBs and RETURNs.
ON ERROR GOTO h
GOSUB g
PRINT "back from g"
se
GOSUB g
PRINT "back from g again"
SYSTEM
g:
PRINT "in g"
RETURN
h:
PRINT "handler"; ERR
RESUME NEXT
SUB se
    GOSUB l
    PRINT "not reached"
    l:
    PRINT "in se l, leaving"
    EXIT SUB
END SUB
