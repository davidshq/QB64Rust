$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB executes RETURN while a GOSUB of the main module is pending.
ON ERROR GOTO h
GOSUB g1
PRINT "back in main"
PRINT "end"
SYSTEM
g1:
PRINT "in g1"
sret
PRINT "g1 after sret"
RETURN
PRINT "after g1 RETURN"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT
SUB sret
    PRINT "in sret"
    RETURN
    PRINT "sret after RETURN"
END SUB
