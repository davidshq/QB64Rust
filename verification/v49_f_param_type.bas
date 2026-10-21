$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a parameter typed by a TYPE in a SUB above, a DECLARE above the block, and an included TYPE.
'$INCLUDE: 'v49_inc/incl.bi'
DECLARE SUB later (p AS middle)
DIM v AS early: v.k = 6: use v: PRINT fn(v)
DIM w(2) AS early: w(1).k = 3: usea w()
DIM m AS middle: m.j = 4: later m
DIM i AS incl: i.z = 5: inc i
SYSTEM
SUB defines
    TYPE early
        k AS LONG
    END TYPE
END SUB
SUB use (p AS early)
    PRINT p.k
END SUB
SUB usea (p() AS early)
    PRINT p(1).k
END SUB
FUNCTION fn& (p AS early)
    fn = p.k * 2
END FUNCTION
TYPE middle
    j AS INTEGER
END TYPE
SUB later (p AS middle)
    PRINT p.j
END SUB
SUB inc (p AS incl)
    PRINT p.z
END SUB
