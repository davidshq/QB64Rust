$CONSOLE:ONLY
' Verification (m2-control-flow-slice, 4.3): OPTION _EXPLICIT, a SUB's own CONST and a main CONST used in the SUB, a FUNCTION read by its name.
OPTION _EXPLICIT
CONST m = 5
s
PRINT f
SYSTEM
SUB s
    CONST k = 2
    PRINT k; m
END SUB
FUNCTION f
    f = 3
    PRINT "f"
END FUNCTION
