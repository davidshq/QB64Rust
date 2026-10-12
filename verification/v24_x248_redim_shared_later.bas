$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): REDIM of an array, then REDIM SHARED of it: is it seen in a
' SUB below, and in one above the second statement?
ON ERROR GOTO h
REDIM ns(3) AS LONG
ns(1) = 5
REDIM SHARED ns(4) AS LONG
ns(2) = 6
PRINT "main:"; UBOUND(ns); ns(1); ns(2)
PRINT "in the SUB:";
show
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB show
    PRINT ns(2);
    PRINT UBOUND(ns)
END SUB
