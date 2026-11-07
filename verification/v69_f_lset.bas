$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which LSET and RSET find their
' target and evaluate their value, and MID$ as a statement its target, start, length and value.
DIM SHARED e(1 TO 3) AS STRING
e(1) = "....": e(2) = "....": e(3) = "...."
PRINT "LSET:";: LSET e(ix(2)) = v$(1): PRINT " ["; e(2); "]"
PRINT "RSET:";: RSET e(ix(3)) = v$(2): PRINT " ["; e(3); "]"
PRINT "MID$:";: MID$(e(ix(1)), ix(2), ix(2)) = v$(3): PRINT " ["; e(1); "]"
SYSTEM

FUNCTION ix& (k AS LONG)
    PRINT " ix"; LTRIM$(STR$(k));
    ix& = k
END FUNCTION

FUNCTION v$ (k AS LONG)
    PRINT " v"; LTRIM$(STR$(k));
    v$ = "ab"
END FUNCTION
