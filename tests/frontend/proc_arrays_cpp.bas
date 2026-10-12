' TEST: cpp
$CONSOLE:ONLY
' Arrays in procedures in the emitter (m2-defxxx-common-arrays task 6.2, design D7): the descriptor of a local
' array is made on each call, "not allocated" (dataK.txt), and its DIM allocates; on every way out of the
' procedure (EXIT SUB and an error go to exit_subfunc, where END SUB arrives too) freeK.txt frees its strings and
' its block, if a statement allocated one, and the lock of its descriptor. A STATIC array and a listed one have
' their descriptors in global.txt and maindata.txt and are not released by the procedure; the STATIC one is
' allocated there too, after its descriptor, by code that runs when the program starts and again when an error
' handler is entered.
SUB locals (n AS LONG)
    DIM t(5) AS LONG, s(n) AS STRING
    t(1) = n
    IF n = 1 THEN EXIT SUB
    s(1) = "a"
END SUB
FUNCTION level& (n AS LONG)
    DIM a(n) AS LONG
    a(n) = n
    IF n > 1 THEN level& = level&(n - 1) + a(n) ELSE level& = a(n)
END FUNCTION
SUB statics
    STATIC c(3) AS LONG
    STATIC l() AS LONG
    DIM l(3) AS LONG
    c(1) = c(1) + l(1)
END SUB
