$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): the whole array x() of a name that is no array under OPTION
' BASE 1 and below $DYNAMIC, each run twice.
ON ERROR GOTO h
OPTION BASE 1
DIM i AS LONG
PRINT "1. OPTION BASE 1, LEN(x()) and bounds:"; LEN(x()); LBOUND(x); UBOUND(x)
'$DYNAMIC
PRINT "2. below $DYNAMIC, LEN(y()) twice:";
FOR i = 1 TO 2
    PRINT LEN(y());
NEXT
PRINT
PRINT "3. below $DYNAMIC, PRINT z() twice:";
FOR i = 1 TO 2
    PRINT z();
NEXT
PRINT

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
