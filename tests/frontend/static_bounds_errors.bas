' TEST: check-fail
$CONSOLE:ONLY
' STATIC arrays whose bounds are not known when compiling (m4-names-implicit-arrays task 5.1, design D4): the old
' compiler writes the bounds into the main module's start code, so a parameter or a variable of the procedure is
' an error (its C++ build fails, verification\v24_x121). A bound whose value is known is accepted, and so is one
' that reads a shared variable or calls a FUNCTION (m4-left-marked, design D1; static_bound_read_typed.bas).
DIM SHARED n AS LONG
SYSTEM

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
    STATIC d(n + p2) AS LONG
END SUB

' A bound that names an array of the procedure (a parameter array, a STATIC one, a local one): not
' measured, so not supported yet.
SUB w (pa() AS LONG)
    STATIC e(UBOUND(pa)) AS LONG
END SUB

SUB x
    STATIC sb(5) AS LONG
    STATIC g(UBOUND(sb)) AS LONG
END SUB

SUB y
    DIM lc(3) AS LONG
    STATIC h(lc(1)) AS LONG
END SUB
