' QB 4.5 (beside SOMEDAY's row on graphics GET and PUT with a FUNCTION in the index, v66_a19): how often the
' index of the array of graphics GET and PUT is evaluated. QB64pe writes it twice, so the FUNCTION runs twice.
' Each call prints its argument, so the number of calls shows on the line.
DECLARE FUNCTION one% (n AS INTEGER)
ON ERROR GOTO h
DIM a(100) AS INTEGER
OPEN "OUT.TXT" FOR OUTPUT AS #1
SCREEN 1
PSET (1, 1), 3
PRINT #1, "1. GET (0, 0)-(3, 3), a(one(1)):";
GET (0, 0)-(3, 3), a(one(1))
PRINT #1, ":"; a(1); a(2)
PRINT #1, "2. PUT (10, 10), a(one(1)):";
PUT (10, 10), a(one(1))
PRINT #1, ":"; POINT(11, 11)
PRINT #1, "3. PUT (20, 20), a(one(1)), PSET:";
PUT (20, 20), a(one(1)), PSET
PRINT #1, ":"; POINT(21, 21)
PRINT #1, "4. GET (0, 0)-(3, 3), a(one(200)):";
GET (0, 0)-(3, 3), a(one(200))
PRINT #1, ":"
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
