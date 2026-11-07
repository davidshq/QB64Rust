$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the arguments of a SUB by reference whose index
' checks raise, beside an argument that raises another error. The first error in the order of the call's
' arguments is the one the handler reports (as verification\v18_c_byref, which Linux's g++ reverses).
DIM x(5) AS LONG, y(5) AS LONG, e AS STRING
ON ERROR GOTO h
PRINT "two x(9), ASC(e)"
two x(9), ASC(e)
PRINT "two ASC(e), x(9)"
two ASC(e), x(9)
PRINT "two x(9), y(9)"
two x(9), y(9)
PRINT "two x(f(1)), y(f(9))"
two x(f(1)), y(f(9))
PRINT "two x(f(9)), y(f(1))"
two x(f(9)), y(f(1))
PRINT "done"
SYSTEM

h:
PRINT " handler"; ERR
RESUME NEXT

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

SUB two (p AS LONG, q AS LONG)
    PRINT " in two"; p; q
END SUB
