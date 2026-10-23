' TEST: check-fail
$CONSOLE:ONLY
' STATIC arrays whose bounds are not known when compiling (m4-names-implicit-arrays task 5.1, design D4): the old
' compiler writes the bounds into the main module's start code, so a parameter or a variable of the procedure is
' an error (its C++ build fails, verification\v24_x121); a shared variable or a FUNCTION, which the start code
' reads at the start and again when a handler is entered, stays marked (v55_x74, x75). A bound whose value is
' known is accepted.
DIM SHARED n AS LONG
SYSTEM

FUNCTION f
    f = 6
END FUNCTION

SUB s (p AS LONG)
    STATIC a(p) AS LONG
END SUB

SUB t
    k = 3
    STATIC b(k + 1) AS LONG
END SUB

SUB u
    STATIC c(2 + 3) AS LONG
END SUB

SUB v
    STATIC d(n) AS LONG
END SUB

SUB w
    STATIC e(f) AS LONG
END SUB
