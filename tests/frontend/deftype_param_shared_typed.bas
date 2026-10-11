' TEST: typed
$CONSOLE:ONLY
' A plain name in a procedure that a DIM SHARED ... AS types in the main module (spec language/default-types;
' measured, v24_a_param_shared): it means the procedure's own variable of that name and the default type (a
' parameter, a local), else the shared one: after a DEFxxx in the body, for another type, for one bound by SHARED.
DIM SHARED x AS LONG
DIM SHARED y AS STRING
DIM SHARED k AS DOUBLE
DIM SHARED m AS LONG
x = 7: y = "main": k = 1.5: m = 70
s 1.5, 2.5
PRINT "1:"; x; y
PRINT "2:"; f(1.5)
PRINT "3:"; x
t 2.5
PRINT "4:"; m
u 2.5
PRINT "5:"; k
w 1
PRINT "6:"; x
x! = 1.25
h 1
PRINT "7:"; x; x!
p 1.5, 2.5
PRINT "8:"; x
SYSTEM

SUB s (x, y)
    PRINT "s:"; x; y
    x = 3.25
END SUB

FUNCTION f (x)
    f = x * 2
    x = 9
END FUNCTION

' The default changes in the body: what the plain name means below the DEFINT.
SUB t (m)
    PRINT "t1:"; m
    DEFINT A-Z
    PRINT "t2:"; m
    m = 9
    PRINT "t3:"; m
END SUB

' Below the DEFINT of t: the parameter is an INTEGER.
SUB u (k)
    PRINT "u:"; k
    k = 4
    PRINT "u:"; k
END SUB

' Still below the DEFINT of t (the default is INTEGER). A local variable written with the default type's suffix:
' the plain name means it, not the shared one.
SUB w (q)
    x% = 3
    PRINT "w:"; x
    x = 2
    PRINT "w:"; x%
END SUB

' A main-module variable bound by SHARED with the default type's suffix: the plain name stays the DIM SHARED one.
DEFSNG A-Z
SUB h (q)
    SHARED x!
    PRINT "h:"; x
    x = 5
END SUB

' Parameters with a suffix: the default type's (the parameter) and another (the shared variable).
SUB p (x!, m%)
    PRINT "p:"; x; m
    x = 6
END SUB
