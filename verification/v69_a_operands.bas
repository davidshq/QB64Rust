$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which the two operands of one
' operator are evaluated. Each FUNCTION prints its name and argument before the result is printed. QB64pe's
' Windows build evaluates them left to right; QB64Rust does so on every platform (spec language/evaluation-order).
DIM SHARED n AS LONG, x AS LONG
PRINT "counter:";: PRINT c(1) - c(2)
PRINT "-:";: PRINT f(1) - f(2)
PRINT "+:";: PRINT f(1) + f(2)
PRINT "*:";: PRINT f(1) * f(2)
PRINT "/:";: PRINT f(1) / f(2)
PRINT "\:";: PRINT f(7) \ f(2)
PRINT "MOD:";: PRINT f(7) MOD f(2)
PRINT "^:";: PRINT f(2) ^ f(3)
PRINT "=:";: PRINT f(1) = f(2)
PRINT "<:";: PRINT f(1) < f(2)
PRINT "AND:";: PRINT f(1) AND f(3)
PRINT "OR:";: PRINT f(1) OR f(2)
PRINT "XOR:";: PRINT f(1) XOR f(3)
PRINT "EQV:";: PRINT f(1) EQV f(2)
PRINT "IMP:";: PRINT f(1) IMP f(2)
PRINT "three:";: PRINT f(1) - f(2) - f(3)
PRINT "nested:";: PRINT f(1) * (f(2) + f(3))
PRINT "string +:";: PRINT s$(1) + s$(2)
PRINT "string =:";: PRINT s$(1) = s$(2)
PRINT "string <:";: PRINT s$(1) < s$(2)
x = 1
PRINT "x + g:";: PRINT x + g
x = 1
PRINT "g + x:";: PRINT g + x
SYSTEM

' Adds 1 to the counter and gives it.
FUNCTION c& (k AS LONG)
    n = n + 1
    PRINT " c"; LTRIM$(STR$(k));
    c& = n
END FUNCTION

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

FUNCTION s$ (k AS LONG)
    PRINT " s"; LTRIM$(STR$(k));
    s$ = CHR$(64 + k)
END FUNCTION

' Sets x to 10 and gives 0.
FUNCTION g&
    PRINT " g";
    x = 10
    g& = 0
END FUNCTION
