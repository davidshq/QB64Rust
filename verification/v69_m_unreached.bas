$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): places of a group that no other program of the
' corpus or the verification families reaches. Each FUNCTION prints its name and argument before the result.
DIM SHARED a(3, 3) AS LONG, v(3) AS LONG
DIM s AS STRING
s = "abcdef"
a(1, 2) = 12
PRINT "MID$:";: PRINT MID$(s, f(2), f(3))
PRINT "INSTR:";: PRINT INSTR(f(1), t$(1), t$(2))
PRINT "_RGB32:";: PRINT _RGB32(f(1), f(2), f(3))
PRINT "a(f(1), f(2)):";: PRINT a(f(1), f(2))
PRINT "CHR$ + CHR$:";: PRINT CHR$(f(65)) + CHR$(f(66))
v(1) = 1: v(2) = 2
PRINT "SWAP:";: SWAP v(f(1)), v(f(2)): PRINT v(1); v(2)
SYSTEM

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

FUNCTION t$ (k AS LONG)
    PRINT " t"; LTRIM$(STR$(k));
    t$ = MID$("bcd", k, 1)
END FUNCTION
