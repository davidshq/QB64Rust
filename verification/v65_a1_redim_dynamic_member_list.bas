$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY, member arrays): a _DYNAMIC member array of an array element beside other targets of one
' REDIM: how often and in what order the indexes are evaluated, and which targets get their new bounds when an
' index is out of range. Each probe is a statement of its own: UBOUND shows the bounds each target holds.
TYPE dy
    id AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
    _DYNAMIC s(0 TO 1) AS STRING
END TYPE
ON ERROR GOTO h
DIM SHARED a(3) AS dy, t AS dy
REDIM SHARED w(2) AS LONG
DIM i AS LONG
fill
PRINT "1. REDIM a(one(1)).d(1 TO 5), t.d(1 TO 6), w(4):";
REDIM a(one(1)).d(1 TO 5) AS LONG, t.d(1 TO 6) AS LONG, w(4) AS LONG
show
fill
PRINT "2. REDIM a(9).d(1 TO 5), t.d(1 TO 6), w(4):";
REDIM a(9).d(1 TO 5) AS LONG, t.d(1 TO 6) AS LONG, w(4) AS LONG
show
fill
PRINT "3. REDIM t.d(1 TO 6), a(9).d(1 TO 5), w(4):";
REDIM t.d(1 TO 6) AS LONG, a(9).d(1 TO 5) AS LONG, w(4) AS LONG
show
fill
PRINT "4. REDIM w(4), a(one(2)).d(1 TO 5), a(one(3)).s(0 TO 7), t.d(1 TO 6):";
REDIM w(4) AS LONG, a(one(2)).d(1 TO 5) AS LONG, a(one(3)).s(0 TO 7) AS STRING, t.d(1 TO 6) AS LONG
show
fill
PRINT "5. REDIM a(9).d(1 TO 5), a(one(1)).d(1 TO 5), a(8).s(0 TO 7), t.d(1 TO 6):";
REDIM a(9).d(1 TO 5) AS LONG, a(one(1)).d(1 TO 5) AS LONG, a(8).s(0 TO 7) AS STRING, t.d(1 TO 6) AS LONG
show
fill
PRINT "6. REDIM a(one(0)).d(1 TO 5), a(one(0)).s(0 TO 7):";
REDIM a(one(0)).d(1 TO 5) AS LONG, a(one(0)).s(0 TO 7) AS STRING
show
fill
PRINT "7. REDIM a(one(1)).d(1 TO 5), a(one(9)).d(1 TO 5), t.d(1 TO 6):";
REDIM a(one(1)).d(1 TO 5) AS LONG, a(one(9)).d(1 TO 5) AS LONG, t.d(1 TO 6) AS LONG
show
fill
PRINT "8. REDIM _PRESERVE a(one(1)).d(1 TO 5), t.d(1 TO 6):";
REDIM _PRESERVE a(one(1)).d(1 TO 5) AS LONG, t.d(1 TO 6) AS LONG
show
PRINT " a(1).d(2)"; a(1).d(2); " t.d(2)"; t.d(2)
fill
PRINT "9. REDIM a(one(1)).d(1 TO one(5)), t.d(1 TO one(6)):";
REDIM a(one(1)).d(1 TO one(5)) AS LONG, t.d(1 TO one(6)) AS LONG
show
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = n
END FUNCTION

' Prints the UBOUND of each member and of w, and a(i).d(2), which a REDIM without _PRESERVE clears.
SUB show
    DIM i AS LONG
    PRINT ":";
    FOR i = 0 TO 3
        PRINT " a("; LTRIM$(STR$(i)); ").d";
        PRINT UBOUND(a(i).d);
        PRINT a(i).d(2);
        PRINT " .s";
        PRINT UBOUND(a(i).s);
    NEXT
    PRINT " t.d";
    PRINT UBOUND(t.d);
    PRINT " w";
    PRINT UBOUND(w)
END SUB

' Gives every member its first bounds and values again, and w its bounds.
SUB fill
    DIM i AS LONG
    FOR i = 0 TO 3
        REDIM a(i).d(1 TO 3) AS LONG
        REDIM a(i).s(0 TO 1) AS STRING
        a(i).d(2) = i + 10
    NEXT
    REDIM t.d(1 TO 3) AS LONG
    t.d(2) = 99
    REDIM w(2) AS LONG
END SUB
