$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): DIM of a type inside a SUB above its TYPE block in the same SUB.
s
SYSTEM

SUB s
    DIM p AS inner
    p.k = 3
    PRINT p.k; LEN(p)
    TYPE inner
        k AS _BYTE
        j AS LONG
    END TYPE
END SUB
