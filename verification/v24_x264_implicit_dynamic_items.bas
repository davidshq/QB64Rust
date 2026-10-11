$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 5): below $DYNAMIC, where the allocation of an array
' used without DIM runs inside a statement of several items, when the array is not named by the first item.
' Each statement runs twice; the second round's allocation raises error 10.
ON ERROR GOTO h
DIM i AS LONG, a AS LONG, b AS LONG
'$DYNAMIC
PRINT "1. PRINT, a text before the first use:";
FOR i = 1 TO 2
    PRINT " x"; dp(1); " y";
NEXT
PRINT
PRINT "2. READ a, dq(1), b:";
FOR i = 1 TO 2
    READ a, dq(1), b
NEXT
PRINT a; dq(1); b
READ a
PRINT "   the next item:"; a
PRINT "3. WRITE i, dr(1):";
FOR i = 1 TO 2
    WRITE i, dr(1)
NEXT
PRINT
PRINT "4. SWAP sv, ds(1):";
sv = 7
FOR i = 1 TO 2
    SWAP sv, ds(1)
NEXT
PRINT sv; ds(1)
PRINT "5. a call with two arguments:";
FOR i = 1 TO 2
    two i, dt(1)
NEXT
PRINT
PRINT "6. a store whose index names it:";
FOR i = 1 TO 2
    dx(dy(1) + i) = i
NEXT
PRINT dx(1); dx(2); UBOUND(dy)
PRINT "7. the first item names it, more follow:";
FOR i = 1 TO 2
    PRINT pa(1); " z";
NEXT
PRINT
PRINT "8. READ whose first target names it:";
FOR i = 1 TO 2
    READ rd(1), a
NEXT
PRINT rd(1); a
PRINT "9. two new arrays, a text between them:";
FOR i = 1 TO 2
    PRINT ea(1); " m"; eb(1);
NEXT
PRINT
PRINT "10. PRINT, the new array in the third item:";
FOR i = 1 TO 2
    PRINT " p"; " q"; pc(1);
NEXT
PRINT

SYSTEM
DATA 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB two (m AS LONG, v)
    PRINT " two"; m; v;
END SUB
