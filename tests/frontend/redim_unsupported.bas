' TEST: check-fail
$CONSOLE:ONLY
' What stays "not supported yet" around dynamic arrays after m2-defxxx-common-arrays task 4.1, each by its own
' message: REDIM _RETAIN, member arrays in REDIM and ERASE, a string as a bound, and in a procedure a REDIM of a
' name without parentheses (a procedure's own arrays compile: proc_arrays_typed.bas). After a declaration that
' was marked (the first REDIM here) a name used without DIM is not made an array (the declaration may have
' declared it): the use, LBOUND of it and an ERASE of it keep their marks (task 5.1, design D5).
n = 3
REDIM _RETAIN r3(2)
REDIM _PRESERVE _RETAIN r4(2)
REDIM it(0).values(0 TO 3)
REDIM sb("a")
DIM SHARED sh(n) AS LONG
z(3) = 4
ERASE z
PRINT LBOUND(q)
ERASE q
ERASE it(0).values
SYSTEM
SUB local
    REDIM e AS LONG
END SUB
