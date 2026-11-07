' TEST: cpp
$CONSOLE:ONLY
' The order of the parts of one expression or call (spec language/evaluation-order): a part that conflicts with a
' later one is held in a seqN temporary first, in the order of the C++ text. A group with no conflict keeps the
' old compiler's C++.
DIM SHARED n AS LONG
DIM x AS LONG, a(9) AS LONG, b(9) AS LONG, i AS LONG, j AS LONG, s$, t$, c AS LONG, e$(9), m(9, 9) AS LONG
' Two FUNCTION calls as operands: the first is held.
PRINT f(1) - f(2)
' A variable before a FUNCTION that may change it: the variable is held.
PRINT x + g
' The arguments of a built-in.
PRINT MID$(s$, f(1), f(2))
' A SUB's arguments by reference, whose index checks raise: the first is held.
two a(f(1)), b(i \ j)
' FIELD and LSET, each one call.
FIELD #f(1), f(2) AS s$
FIELD #1, f(2) AS e$(f(3))
LSET s$ = MID$(t$, f(3))
' A numeric store into an element: the index is a statement of its own, written first; nothing held.
a(f(1)) = f(2)
' Nothing held: two index checks, a string function that cannot raise, constants.
PRINT a(i) + b(j)
PRINT s$ + LEFT$(t$, 1)
PRINT 1 + f(1)
' _IIF and _ANDALSO: nothing is moved out of a branch.
PRINT _IIF(c, f(1) + f(2), f(3))
PRINT c _ANDALSO f(1) - f(2)
' A constant index before a FUNCTION index: its check is held, because it raises error 9 and the FUNCTION may
' raise another error. The descriptor's reads alone do not count.
PRINT m(1, f(1))
SYSTEM

FUNCTION f& (k AS LONG)
    n = n + 1
    f& = n
END FUNCTION

FUNCTION g&
    SHARED x AS LONG
    g& = 0
END FUNCTION

SUB two (p AS LONG, q AS LONG)
END SUB
