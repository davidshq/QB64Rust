$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY, member arrays): a _DYNAMIC member array of an array element as the only target of ERASE,
' beside v64_a1: how often the index is evaluated, and which element an index out of range releases.
TYPE dy
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
ON ERROR GOTO h
DIM SHARED a(3) AS dy
PRINT "1. ERASE a(one(1)).d:";
ERASE a(one(1)).d
show
PRINT "2. ERASE a(9).d:";
ERASE a(9).d
show
PRINT "3. ERASE a(one(9)).d:";
REDIM a(0).d(1 TO 3) AS LONG
ERASE a(one(9)).d
show
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = n
END FUNCTION

SUB show
    DIM i AS LONG
    PRINT ":";
    FOR i = 0 TO 3
        PRINT " a("; LTRIM$(STR$(i)); ").d";
        PRINT UBOUND(a(i).d);
    NEXT
    PRINT
END SUB
