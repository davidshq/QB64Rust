$CONSOLE:ONLY
' Verification: accidental behaviours from study\02 section 9.2.
ON ERROR GOTO handler

PRINT "== _BIT * 33 scalars (8-byte C type, 4 bytes reserved) =="
DIM a AS _BIT * 33, b AS _BIT * 33
a = 5
b = -1
PRINT a; b
a = 4294967295 \ 2
PRINT a; b

PRINT "== static SELECT CASE temporary in recursion =="
' Correct answer is 0 for every n; 1 means the outer SELECT saw the inner call's value.
FOR n% = 0 TO 3: PRINT f%(n%);: NEXT: PRINT

PRINT "== ON n GOTO range (QB4.5: error 5 for n < 0 or n > 255) =="
FOR k% = -1 TO 2
    PRINT "n ="; k%;
    ON k% GOTO l1, l2
    PRINT " fell through"
    GOTO nextk
    l1: PRINT " -> l1": GOTO nextk
    l2: PRINT " -> l2"
    nextk:
NEXT
k% = 256: PRINT "n = 256";
ON k% GOTO l3
PRINT " fell through"
l3:
k% = 257: PRINT "n = 257 (257 AND 255 = 1)";
ON k% GOTO l4
PRINT " fell through"
GOTO done
l4: PRINT " -> l4"
done:
SYSTEM

handler:
PRINT " trapped error"; ERR
RESUME NEXT

FUNCTION f% (n%)
    SELECT CASE n% + 0
        CASE probe%(n%)
            f% = -1
        CASE n% - 1
            f% = 1
        CASE ELSE
            f% = 0
    END SELECT
END FUNCTION

FUNCTION probe% (n%)
    IF n% > 0 THEN dummy% = f%(n% - 1)
    probe% = -99
END FUNCTION
