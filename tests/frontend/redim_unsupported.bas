' TEST: check-fail
$CONSOLE:ONLY
' What stays "not supported yet" around dynamic arrays after m2-defxxx-common-arrays task 4.1, each by its own
' message: REDIM and ERASE of a member of a name that is no array (member arrays compile:
' member_arrays_typed.bas), a string as a bound, bounds too far apart, and in a procedure a REDIM of a
' name without parentheses (a procedure's own arrays compile: proc_arrays_typed.bas). After a declaration that
' was marked (the first REDIM here) a name used without DIM is not made an array (the declaration may have
' declared it): the use, LBOUND of it and an ERASE of it keep their marks (task 5.1, design D5). REDIM _RETAIN
' compiles since m3-redim-retain (redim_retain_*.bas).
n = 3
REDIM it(0).values(0 TO 3)
REDIM sb("a")
DIM SHARED sh(n) AS LONG
z(3) = 4
ERASE z
PRINT LBOUND(q)
ERASE q
ERASE it(0).values
' Bounds whose distance does not fit 64 bits (not measured; a compiler crash until 2026-10-12).
DIM wide(-9223372036854775808 TO 9223372036854775807) AS LONG
SYSTEM
SUB local
    REDIM e AS LONG
END SUB
