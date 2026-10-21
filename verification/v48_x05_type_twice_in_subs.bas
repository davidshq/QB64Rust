$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): two SUBs that each define TYPE pt.
s1
s2
SYSTEM

SUB s1
    TYPE pt
        x AS LONG
    END TYPE
    DIM p AS pt: p.x = 1: PRINT p.x; LEN(p)
END SUB

SUB s2
    TYPE pt
        x AS INTEGER
    END TYPE
    DIM p AS pt: p.x = 2: PRINT p.x; LEN(p)
END SUB
