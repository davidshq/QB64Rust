' TEST: typed
$CONSOLE:ONLY
' Argument passing (m2-procedures-and-errors D4, spec language/procedures "Argument passing"): each argument form
' is shown as `ref` (the variable itself) or `temp` (a converted copy)
DIM n AS LONG, i AS INTEGER, s AS STRING
CALL bump(n)
bump n
bump (n)
CALL bump((n))
bump n + 1
bump i
bump 2.5
bump x&
greet s, n
greet (s), 2.5
greet s + "!", i
greet "lit", -n
PRINT f&(n); f&((n)); f&(i)
SUB bump (x AS LONG)
    x = x + 1
    bump x
END SUB
SUB greet (t AS STRING, k AS INTEGER)
    PRINT t; k
END SUB
FUNCTION f& (a AS LONG)
    f = a
END FUNCTION
