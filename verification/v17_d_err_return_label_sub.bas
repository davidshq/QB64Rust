$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): RETURN label inside a SUB, naming a label of the SUB.
t
SYSTEM
SUB t
    GOSUB g
    back:
    PRINT "back"
    EXIT SUB
    g:
    RETURN back
END SUB
