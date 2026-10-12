$CONSOLE:ONLY
' COMMON (spec language/arrays-and-types, "COMMON"): COMMON declares each variable as DIM does and COMMON SHARED as
' DIM SHARED does, wherever it stands in the main module. COMMON name() lists a dynamic array that is not
' allocated; what stands between its parentheses is not read. A DIM of that name and type below allocates it when
' it runs, a REDIM gives it new bounds, and a use before either raises error 9. Error cases go to a handler that
' prints ERR and resumes next.
TYPE pt
    x AS LONG
END TYPE
ON ERROR GOTO h
DIM n AS LONG, i AS LONG
n = 3
COMMON SHARED total AS LONG, word$, w AS pt, fx AS STRING * 4
COMMON v AS LONG
COMMON AS LONG la, lb()
COMMON SHARED cs() AS LONG, ign(3) AS LONG, other()
COMMON sc, never() AS DOUBLE, twice() AS LONG
total = 5: word$ = "w": w.x = 6: fx = "abcdef": v = 7
PRINT "1. COMMON SHARED scalars, seen in a SUB:";
scalars
PRINT "2. COMMON alone is not shared:";
unshared
PRINT "3. COMMON cs(), used before its DIM:";
cs(2) = 6
PRINT
DIM cs(n) AS LONG
cs(2) = 6
PRINT "4. COMMON SHARED cs() then DIM cs(n), in a SUB:";
arrays
PRINT "5. REDIM of it:";
REDIM cs(9) AS LONG
PRINT UBOUND(cs); cs(2)
PRINT "6. COMMON ign(3) allocates nothing:";
PRINT UBOUND(ign);
PRINT
DIM SHARED ign(2, 5) AS LONG
ign(1, 2) = 8
PRINT "7. DIM SHARED ign(2, 5) below it:"; UBOUND(ign, 1); UBOUND(ign, 2);
arrays2
DIM other(3) AS LONG
other(2) = 6
PRINT "8. COMMON SHARED other() then DIM other(3) AS LONG, in a SUB:";
another
PRINT
PRINT "9. COMMON never(), never given bounds:";
never(1) = 1
PRINT UBOUND(never);
PRINT
PRINT "10. the type before the names:";
DIM lb(1 TO 2) AS LONG
la = 100000: lb(2) = 200000
PRINT la; lb(2); LBOUND(lb)
DIM sc(3)
sc = 1.5: sc(1) = 2.5
PRINT "11. COMMON sc then DIM sc(3):"; sc; sc(1)
PRINT "12. DIM of a COMMON array, run twice:";
FOR i = 1 TO 2
    DIM twice(i + 1) AS LONG
NEXT
PRINT UBOUND(twice)
COMMON SHARED below
below = 9
PRINT "13. COMMON below statements:";
late
ERASE cs
PRINT "14. ERASE of a COMMON array:";
PRINT UBOUND(cs);
PRINT
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB scalars
    PRINT total; word$; w.x; fx
    total = total + 1
END SUB
SUB unshared
    PRINT v
END SUB
SUB arrays
    PRINT cs(2); UBOUND(cs)
END SUB
SUB arrays2
    PRINT ign(1, 2)
END SUB
SUB another
    PRINT other(2);
END SUB
SUB late
    PRINT below
END SUB
