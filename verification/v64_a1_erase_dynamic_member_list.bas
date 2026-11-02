$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY, member arrays): a _DYNAMIC member array of an array element beside other targets of one
' ERASE: how often and in what order the indexes are evaluated, and which targets are released when an index is
' out of range. Each probe is a statement of its own: UBOUND of a released member raises error 9.
TYPE dy
    id AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
    _DYNAMIC s(0 TO 1) AS STRING
END TYPE
ON ERROR GOTO h
DIM SHARED a(3) AS dy, t AS dy, w(2) AS LONG
DIM i AS LONG
FOR i = 0 TO 3: a(i).d(2) = i + 10: a(i).s(1) = "s" + STR$(i): NEXT
t.d(2) = 99: w(1) = 7
PRINT "1. ERASE a(one(1)).d, t.d, w:";
ERASE a(one(1)).d, t.d, w
show
fill
PRINT "2. ERASE a(9).d, t.d, w:";
ERASE a(9).d, t.d, w
show
fill
PRINT "3. ERASE t.d, a(9).d, w:";
ERASE t.d, a(9).d, w
show
fill
PRINT "4. ERASE w, a(one(2)).d, a(one(3)).s, t.d:";
ERASE w, a(one(2)).d, a(one(3)).s, t.d
show
fill
PRINT "5. ERASE a(9).d, a(one(1)).d, a(8).s, t.d:";
ERASE a(9).d, a(one(1)).d, a(8).s, t.d
show
fill
PRINT "6. ERASE a(one(0)).d, a(one(0)).s:";
ERASE a(one(0)).d, a(one(0)).s
show
fill
PRINT "7. ERASE a(one(1)).d, a(one(9)).d, t.d:";
ERASE a(one(1)).d, a(one(9)).d, t.d
show
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = n
END FUNCTION

' Prints what each target holds: the UBOUND of each member (error 9 when released) and w(1) (0 when cleared).
SUB show
    DIM i AS LONG
    PRINT ":";
    FOR i = 0 TO 3
        PRINT " a("; LTRIM$(STR$(i)); ").d";
        PRINT UBOUND(a(i).d);
        PRINT " .s";
        PRINT UBOUND(a(i).s);
    NEXT
    PRINT " t.d";
    PRINT UBOUND(t.d);
    PRINT " w(1)"; w(1)
END SUB

' Gives every member its bounds again and w(1) its value.
SUB fill
    DIM i AS LONG
    FOR i = 0 TO 3
        REDIM a(i).d(1 TO 3) AS LONG
        REDIM a(i).s(0 TO 1) AS STRING
    NEXT
    REDIM t.d(1 TO 3) AS LONG
    w(1) = 7
END SUB
