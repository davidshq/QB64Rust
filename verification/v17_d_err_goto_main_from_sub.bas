$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOTO from a SUB to a label that stands only in the main module.
t
done:
PRINT "done"
SYSTEM
SUB t
    GOTO done
END SUB
