$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOSUB from a SUB to a label that stands only in the main module.
t
SYSTEM
g:
PRINT "g"
RETURN
SUB t
    GOSUB g
END SUB
