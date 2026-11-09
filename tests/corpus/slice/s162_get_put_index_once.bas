$CONSOLE
$SCREENHIDE
' Slice (QB64PE-DIVERGENCES.md D-061): a FUNCTION in the index of the array of graphics GET and PUT is called
' once, as in QB 4.5 and PDS 7.1 (verification\qb45\q45_get_put_index_call). QB64pe calls it twice
' (verification\v66_a19). An array of numbers, an element of an array of strings and of a TYPE, a member of
' one, two dimensions, and an index outside the bounds. Lines [9] and [10] have a FUNCTION in the omit color
' too, after the array: the index's call comes first, left to right. Each call of f% adds its argument to log$;
' the log and the bytes are printed at the end, on the console. Recorded from the program with each index
' stored in k first.
TYPE pt
    x AS INTEGER
    y AS INTEGER
END TYPE
DIM SHARED log$
DIM a(100) AS INTEGER, b(3, 3) AS INTEGER, s(3) AS STRING, r(3) AS pt
ON ERROR GOTO h
SCREEN 13
PSET (1, 1), 4
PSET (2, 1), 5
s(1) = SPACE$(8)
log$ = log$ + " [1]": GET (1, 1)-(2, 1), a(f%(1))
log$ = log$ + " [2]": PUT (10, 10), a(f%(1))
log$ = log$ + " [3]": PUT (20, 20), a(f%(1)), PSET
log$ = log$ + " [4]": GET (1, 1)-(2, 1), b(f%(1), f%(2))
log$ = log$ + " [5]": GET (1, 1)-(2, 1), s(f%(1))
log$ = log$ + " [6]": GET (1, 1)-(2, 1), r(f%(2))
log$ = log$ + " [7]": GET (1, 1)-(2, 1), r(f%(3)).y
log$ = log$ + " [8]": GET (1, 1)-(2, 1), a(f%(200))
log$ = log$ + " [9]": PUT (30, 30), a(f%(1)), PSET, f%(2)
log$ = log$ + " [10]": PUT (40, 40), r(f%(2)), PSET, f%(3)
p1 = POINT(11, 10): p2 = POINT(21, 20): p3 = POINT(31, 30): p4 = POINT(41, 40)
_DEST _CONSOLE
PRINT log$
PRINT a(1); a(2); a(3); b(1, 2); b(2, 2)
PRINT LEN(s(1)); ASC(s(1), 5); ASC(s(1), 6)
PRINT r(2).x; r(2).y; r(3).y
PRINT p1; p2; p3; p4
SYSTEM
h:
log$ = log$ + " (error" + STR$(ERR) + ")"
RESUME NEXT

FUNCTION f% (n AS INTEGER)
    log$ = log$ + STR$(n)
    f% = n
END FUNCTION
