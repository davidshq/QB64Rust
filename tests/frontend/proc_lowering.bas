' TEST: ir
$CONSOLE:ONLY
' Lowering pairs for procedures (design D6): each argument form, a FUNCTION in a PRINT, EXIT, storage classes
DIM n AS LONG, i AS INTEGER
' by reference: a plain variable of the parameter's type; a copy: parentheses, an expression, another type
CALL bump(n): bump n: bump (n): bump n + 1: bump i
' strings: a variable (also in parentheses) by reference; an expression or a literal as a temporary
greet s$, 2.5: greet (s$), n: greet s$ + "!", i: greet "lit", 1
PRINT twice&(n); twice&(twice&(2)); dbl$("x")
counter
SYSTEM

SUB bump (x AS LONG)
    x = x + 1
    y& = x
    EXIT SUB
END SUB

SUB greet (t AS STRING, k AS INTEGER)
    SHARED n AS LONG
    DIM local AS STRING
    local = t + "?"
    PRINT local; k; n
END SUB

FUNCTION twice& (a AS LONG)
    twice = a * 2
    EXIT FUNCTION
END FUNCTION

FUNCTION dbl$ (t AS STRING)
    dbl$ = t + t
END FUNCTION

SUB counter
    STATIC c AS LONG, acc$
    c = c + 1
    acc$ = acc$ + "a"
END SUB
