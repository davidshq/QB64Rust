' TEST: ir
$CONSOLE:ONLY
' Arrays in procedures in the IR (m2-defxxx-common-arrays task 6.1; spec compiler/pipeline "Array storage in the
' IR"): a procedure's own array is a local dynamic array and its DIM an Allocate in mode Dim, also with constant
' bounds; a STATIC array is a dynamic one that lives for the program, with the bounds it is allocated with when
' the program starts (no operation allocates it); a listed array (STATIC l()) is dynamic and its DIM an Allocate;
' the arrays of a procedure with STATIC after its header live for the program, static with constant bounds.
' Nothing says when a local array is released: the emitter does that on every way out of the procedure.
DIM g(3) AS LONG
g(1) = 2
SUB locals (n AS LONG)
    DIM t(5) AS LONG, s(n) AS STRING
    t(1) = n
    IF n = 1 THEN EXIT SUB
    REDIM _PRESERVE t(n) AS LONG
    ERASE s
END SUB
SUB statics
    STATIC c(3) AS LONG
    STATIC l() AS LONG
    DIM l(3) AS LONG
    c(1) = c(1) + l(1)
    ERASE c, l
END SUB
SUB shares
    SHARED g() AS LONG
    g(2) = g(1)
END SUB
SUB keeps (n AS LONG) STATIC
    DIM a(2) AS LONG, b(n) AS LONG
    k = k + a(1)
    u(1) = k
END SUB
