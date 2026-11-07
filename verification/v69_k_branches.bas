$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): _IIF, _ANDALSO and _ORELSE with FUNCTIONs in
' both parts. Only the part chosen, or needed, is evaluated, and a group inside it keeps its order.
DIM c AS LONG
c = -1
PRINT "_IIF true:";: PRINT _IIF(c, f(1), f(2))
c = 0
PRINT "_IIF false:";: PRINT _IIF(c, f(1), f(2))
c = -1
PRINT "_IIF with groups:";: PRINT _IIF(c, f(1) - f(2), f(3) - f(4))
PRINT "_ANDALSO true:";: PRINT f(1) _ANDALSO f(2) - f(3)
PRINT "_ANDALSO false:";: PRINT f(0) _ANDALSO f(2) - f(3)
PRINT "_ORELSE true:";: PRINT f(1) _ORELSE f(2) - f(3)
PRINT "_ORELSE false:";: PRINT f(0) _ORELSE f(2) - f(3)
PRINT "inside an operand:";: PRINT f(1) + _IIF(f(2), f(3), f(4))
SYSTEM

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION
