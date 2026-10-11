$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a DIM SHARED, a REDIM SHARED and a COMMON SHARED array named in
' a SUB that stands above the declaration in the file, and in one below it.
SUB above
    PRINT "SUB above the declarations: a(1), r(1), c(1):";
    PRINT a(1);
    PRINT r(1);
    PRINT c(1)
END SUB
COMMON SHARED c() AS LONG
ON ERROR GOTO h
DIM SHARED a(3) AS LONG
REDIM SHARED r(3) AS LONG
DIM c(3) AS LONG
a(1) = 1: r(1) = 2: c(1) = 3
above
below

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB below
    PRINT "SUB below the declarations: a(1), r(1), c(1):";
    PRINT a(1);
    PRINT r(1);
    PRINT c(1)
END SUB
