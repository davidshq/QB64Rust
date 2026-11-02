' QB 4.5 (v64_a1, a2, beside SOMEDAY's member-array rows): how often the index of an element of an array of a
' TYPE is evaluated where the element is stored, read, swapped and passed, and the store after an index out
' of range. QB 4.5 has no arrays inside a TYPE, so this is the nearest form.
DECLARE FUNCTION one% (n AS INTEGER)
TYPE t1
    m AS INTEGER
END TYPE
ON ERROR GOTO h
DIM SHARED a(3) AS t1
DIM b(3) AS INTEGER
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "1. a(one(1)).m = 5:";
a(one(1)).m = 5
PRINT #1, ":"; a(1).m
PRINT #1, "2. PRINT a(one(1)).m:";
PRINT #1, a(one(1)).m
PRINT #1, "3. SWAP a(one(1)).m, a(one(2)).m:";
SWAP a(one(1)).m, a(one(2)).m
PRINT #1, ":"; a(1).m; a(2).m
PRINT #1, "4. a(9).m = 6:";
a(9).m = 6
PRINT #1, ":"; a(0).m; a(3).m
PRINT #1, "5. b(one(1)) = 7:";
b(one(1)) = 7
PRINT #1, ":"; b(1)
PRINT #1, "6. a(one(9)).m = 8:";
a(one(9)).m = 8
PRINT #1, ":"; a(0).m
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, " [error"; ERR; "]";
RESUME NEXT

FUNCTION one% (n AS INTEGER)
    PRINT #1, n;
    one% = n
END FUNCTION
