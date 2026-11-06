$CONSOLE:ONLY
' v68 "Unused variable": which variables qb64pe -w reports, and at which line. One case per line.
DIM dimonly AS INTEGER
PRINT readonce
setonce = 1
setread = 2: PRINT setread
FOR counter = 1 TO 2: NEXT
FOR counted = 1 TO 2: NEXT counted
DIM bounded(5) AS INTEGER: PRINT LBOUND(bounded)
PRINT implicitarr(1)
DIM dimarr(3) AS LONG
x$ = "a": PRINT x
PRINT Twice(3)
Takes 1
SYSTEM

FUNCTION Twice (n)
    Twice = n * 2
END FUNCTION

FUNCTION NoResult (m)
    PRINT "never called"
END FUNCTION

SUB Takes (unusedparam)
    DIM localonly AS DOUBLE
    PRINT "takes"
END SUB
