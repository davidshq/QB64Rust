' TEST: ir
$CONSOLE:ONLY
' A label in a SUB is a position in the SUB's body; main's labels keep their own numbering, so the handler label
' after them is still found (m2-control-flow-slice task 5.1)
ON ERROR GOTO h
t
start:
PRINT 1
SYSTEM
h:
RESUME NEXT
SUB t
    top:
    PRINT 2
    here: PRINT 3
    last:
END SUB
