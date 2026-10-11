$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): SUB ... STATIC and FUNCTION ... STATIC: scalars used without a
' declaration, declared with DIM, a string and a TYPE variable keep their values; STATIC x(2) and a string
' array inside one; ERASE of its DIMmed array.
TYPE pt
    x AS LONG
END TYPE
ON ERROR GOTO h
DIM v AS LONG
k: k: k
PRINT
v = f(1): v = f(1)
PRINT v
e: e

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB k STATIC
    DIM d AS LONG, p AS pt
    STATIC x(2) AS LONG
    DIM w(2) AS STRING
    n = n + 1: d = d + 2: s$ = s$ + "a": p.x = p.x + 3: x(1) = x(1) + 4: w(1) = w(1) + "b"
    PRINT n; d; s$; p.x; x(1); w(1); "|";
END SUB
FUNCTION f (a AS LONG) STATIC
    c = c + a
    f = c
    PRINT "f:"; c;
END FUNCTION
SUB e STATIC
    DIM q(2) AS LONG
    q(1) = q(1) + 1
    PRINT q(1);
    ERASE q
    PRINT q(1);
END SUB
