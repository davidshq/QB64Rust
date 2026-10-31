$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (member arrays): the order of a REDIM of a _DYNAMIC member array of an array element, alone: its
' bounds and its index, an error each may raise (error 5 for an empty dimension, error 9 for an index outside, a
' bound that raises), and a FUNCTION in a bound that gives the same member other bounds or erases it.
TYPE dy
    id AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
ON ERROR GOTO h
DIM SHARED a(3) AS dy, w(2) AS LONG
fill
PRINT "1. REDIM a(one(1)).d(one(2) TO one(5)):";
REDIM a(one(1)).d(one(2) TO one(5)) AS LONG
show
fill
PRINT "2. REDIM a(one(1)).d(one(2) TO one(5), one(3) TO one(4)):";
REDIM a(one(1)).d(one(2) TO one(5), one(3) TO one(4)) AS LONG
show2
fill
PRINT "3. REDIM a(one(9)).d(1 TO one(5)):";
REDIM a(one(9)).d(1 TO one(5)) AS LONG
show
fill
PRINT "4. REDIM a(one(1)).d(1 TO one(0)):";
REDIM a(one(1)).d(1 TO one(0)) AS LONG
show
fill
PRINT "5. REDIM a(one(9)).d(1 TO one(0)):";
REDIM a(one(9)).d(1 TO one(0)) AS LONG
show
fill
PRINT "6. REDIM a(one(1)).d(1 TO w(one(7))):";
REDIM a(one(1)).d(1 TO w(one(7))) AS LONG
show
fill
PRINT "7. REDIM a(one(9)).d(1 TO w(one(7))):";
REDIM a(one(9)).d(1 TO w(one(7))) AS LONG
show
fill
PRINT "8. REDIM a(1).d(1 TO gone(5)):";
REDIM a(1).d(1 TO gone(5)) AS LONG
show
fill
PRINT "9. REDIM a(1).d(1 TO two(5)):";
REDIM a(1).d(1 TO two(5)) AS LONG
show
fill
PRINT "10. REDIM _PRESERVE a(1).d(1 TO grow(5)):";
REDIM _PRESERVE a(1).d(1 TO grow(5)) AS LONG
show
PRINT " a(1).d(2)"; a(1).d(2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = n
END FUNCTION

' Erases the member being given new bounds.
FUNCTION gone (n AS LONG)
    ERASE a(1).d
    PRINT " gone";
    gone = n
END FUNCTION

' Gives the member being given new bounds two dimensions.
FUNCTION two (n AS LONG)
    REDIM a(1).d(1 TO 2, 1 TO 2) AS LONG
    PRINT " two";
    two = n
END FUNCTION

' Gives the member being given new bounds other bounds and a value.
FUNCTION grow (n AS LONG)
    REDIM a(1).d(1 TO 8) AS LONG
    a(1).d(2) = 77
    PRINT " grow";
    grow = n
END FUNCTION

' Prints the bounds of each member (error 9 when it has none) and a(i).d(2) where it has one.
SUB show
    DIM i AS LONG
    PRINT ":";
    FOR i = 0 TO 3
        PRINT " a("; LTRIM$(STR$(i)); ").d";
        PRINT LBOUND(a(i).d);
        PRINT UBOUND(a(i).d);
    NEXT
    PRINT
END SUB

' The same for two dimensions of a(1).d.
SUB show2
    PRINT ": a(1).d"; LBOUND(a(1).d, 1); UBOUND(a(1).d, 1); LBOUND(a(1).d, 2); UBOUND(a(1).d, 2)
    show
END SUB

' Gives every member its first bounds and values again.
SUB fill
    DIM i AS LONG
    FOR i = 0 TO 3
        REDIM a(i).d(1 TO 3) AS LONG
        a(i).d(2) = i + 10
    NEXT
END SUB
