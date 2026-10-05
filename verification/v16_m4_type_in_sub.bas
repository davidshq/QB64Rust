$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): a TYPE block inside a SUB.
s
SUB s
    TYPE t
        a AS LONG
    END TYPE
    DIM v AS t
    v.a = 2: PRINT v.a
END SUB
SYSTEM
