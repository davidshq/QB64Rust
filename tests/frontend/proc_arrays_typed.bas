' TEST: typed
$CONSOLE:ONLY
' Arrays in procedures (m2-defxxx-common-arrays task 6.1, design D6; measured, verification\v24_g_local,
' v24_g_shared, v24_g_shared_order, v24_x176, x177, x270 to x279). A procedure's own array is local and dynamic,
' also with constant bounds: every DIM and REDIM of it is an Allocate. STATIC a(n) is a dynamic array that lives
' for the program and is allocated when the program starts (no statement allocates it; ERASE releases it and a
' REDIM gives it other bounds), below $DYNAMIC too; STATIC a() lists a dynamic one that lives for the program and
' has no dimensions until a DIM, a REDIM, an index or LBOUND gives them. SHARED a() names the main module's array.
DIM g(3) AS LONG, own(4) AS LONG
DIM SHARED sh(3) AS LONG
REDIM SHARED sd(3) AS LONG
DIM SHARED sv AS LONG
m(2) = 4
SUB locals (n AS LONG)
    DIM t(5) AS LONG, v(n) AS LONG, s(1 TO 2) AS STRING
    REDIM q(n)
    REDIM _PRESERVE q(n + 1)
    t(1) = v(n) + q(1)
    ERASE t
END SUB
SUB statics
    STATIC c(3) AS LONG, w(1 TO 2) AS STRING
    STATIC l() AS LONG
    DIM l(3) AS LONG
    STATIC r()
    REDIM r(2, 3)
    STATIC i() AS DOUBLE
    i(1, 2) = c(1) + l(1) + r(1, 1)
    STATIC b()
    PRINT LBOUND(b)
    ERASE c, l
END SUB
' A use, then DIM: the same local array. DIM a() and REDIM d() declare scalars, as in the main module.
SUB later
    a(1) = 5
    DIM a(3)
    DIM e() AS LONG
    REDIM d() AS LONG
    e = d
END SUB
' SHARED a(): the main module's array, declared or used without DIM; a list; twice; below a local array of the
' name the local one stays. A main array that is not shared is another, local array here.
SUB shares
    SHARED g() AS LONG
    SHARED m(), sh() AS LONG
    SHARED g() AS LONG
    g(1) = m(1)
    DIM own(2) AS LONG
    SHARED own() AS LONG
    own(1) = 1
END SUB
' DIM in a procedure of a shared array of the main module, also with constant bounds: an Allocate of that array
' (it raises error 10 when it runs).
SUB again
    DIM sh(5) AS LONG
    DIM sd(5) AS LONG
    REDIM sd(6) AS LONG
END SUB
' DIM and REDIM of an array that STATIC c(3) declared: an Allocate of it each (the DIM raises error 10 when it
' runs, verification\v24_x285).
SUB dimstatic
    STATIC c(3) AS LONG
    REDIM c(4) AS LONG
    DIM c(3) AS LONG
END SUB
' STATIC after the header: every variable and array lives for the program, a DIM with constant bounds makes a
' static array, an array used without DIM is a static one; a DIM of a DIM SHARED variable's name makes the
' procedure's own variable, which lives for the program too (verification\v24_x284). Below the first STATIC
' statement the procedure is one like any other: its variables and arrays are local again.
SUB keeps (n AS LONG) STATIC
    DIM a(2) AS LONG, b(n) AS LONG
    DIM sv
    sv = sv + 1.5
    k = k + 1
    u(1) = a(1)
    STATIC x(2) AS LONG
    DIM z(2) AS LONG
    j = j + 1
    y(1) = z(1)
END SUB
FUNCTION kept (n AS LONG) STATIC
    c = c + n
    kept = c
END FUNCTION
' Below $DYNAMIC: STATIC a(n) as above; a DIM with constant bounds in a procedure with STATIC after its header is
' a dynamic array that lives for the program, and an array used without DIM there one that nothing allocates.
'$DYNAMIC
SUB dyn STATIC
    DIM a(2) AS LONG
    u(1) = a(1)
    STATIC x(2, 1 TO 3) AS LONG
    REDIM x(4, 4) AS LONG
END SUB
