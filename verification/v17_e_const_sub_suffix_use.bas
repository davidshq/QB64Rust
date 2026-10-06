$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a later SUB uses a main CONST with suffixes.
CONST k = 4
s
SYSTEM
SUB s
    PRINT k%; k&
END SUB
