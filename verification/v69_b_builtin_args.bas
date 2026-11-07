$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which the arguments of a built-in
' function are evaluated, and of built-ins nested in one expression. Each FUNCTION prints its name and argument
' before the result is printed.
PRINT "_MIN:";: PRINT _MIN(pa, pb)
PRINT "_MAX:";: PRINT _MAX(f(1), f(2))
PRINT "_CLAMP:";: PRINT _CLAMP(f(5), f(1), f(9))
PRINT "_ROL:";: PRINT _ROL(f(1), f(2))
PRINT "ASC:";: PRINT ASC(s$(1), f(2))
PRINT "STRING$:";: PRINT STRING$(f(3), s$(4))
PRINT "LEFT$ + RIGHT$:";: PRINT LEFT$(s$(1), f(2)) + RIGHT$(s$(3), f(4))
PRINT "_RGB32:";: PRINT _RGB32(f(1), f(2), f(3))
PRINT "_RGBA32:";: PRINT _RGBA32(f(1), f(2), f(3), f(4))
PRINT "_HYPOT:";: PRINT _HYPOT(f(3), f(4))
PRINT "_ATAN2:";: PRINT _ATAN2(f(1), f(2))
PRINT "LEN + LEN:";: PRINT LEN(s$(1)) + LEN(s$(2))
PRINT "_STRCMP:";: PRINT _STRCMP(s$(1), s$(2))
SYSTEM

FUNCTION pa&
    PRINT " (a)";
    pa& = 1
END FUNCTION

FUNCTION pb&
    PRINT " (b)";
    pb& = 2
END FUNCTION

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

FUNCTION s$ (k AS LONG)
    PRINT " s"; LTRIM$(STR$(k));
    s$ = "abcdef"
END FUNCTION
