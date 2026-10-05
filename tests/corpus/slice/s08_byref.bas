$CONSOLE:ONLY
' Argument passing (spec language/procedures, "Argument passing"): a plain variable of exactly the parameter's type
' is passed by reference; anything else (parentheses, an expression, another type) is converted as for an
' assignment and passed as a copy whose changes are lost.
DIM n AS LONG
DIM i AS INTEGER
DIM d AS DOUBLE
n = 1
CALL bump(n)
PRINT "after CALL bump(n):"; n
bump n
PRINT "after bump n:"; n
bump (n)
PRINT "after bump (n):"; n
bump n + 1
PRINT "after bump n + 1:"; n
CALL bump((n))
PRINT "after CALL bump((n)):"; n
x& = 10
bump x&
PRINT "after bump x&:"; x&
i = 5
bump i
PRINT "INTEGER after bump i:"; i
d = 2.5
bump d
PRINT "DOUBLE after bump d:"; d
showint 2.5
showint 3.5
showint d
showint 40000.4
n = 1
twice n
PRINT "after twice n:"; n
s$ = "abc"
CALL addbang(s$)
PRINT "after CALL addbang(s$): "; s$
addbang (s$)
PRINT "after addbang (s$): "; s$
addbang s$ + "?"
PRINT "after addbang s$ + ?: "; s$
addbang "lit"
greet s$, 2.5
greet "x" + s$, n
SYSTEM

SUB bump (x AS LONG)
    x = x + 1
    PRINT " in bump:"; x
END SUB

SUB showint (v AS INTEGER)
    PRINT "showint:"; v
END SUB

SUB twice (y AS LONG)
    bump y
    bump y
END SUB

SUB addbang (t AS STRING)
    t = t + "!"
    PRINT " in addbang: "; t
END SUB

SUB greet (t AS STRING, k AS INTEGER)
    PRINT "greet: "; t; k
END SUB
