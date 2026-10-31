$CONSOLE
$SCREENHIDE
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): a FUNCTION in the index of the array of graphics GET and PUT: how often it is called.
DIM SHARED calls AS INTEGER
DIM a(100) AS INTEGER
SCREEN 13
PSET (1, 2), 4
GET (1, 2)-(3, 4), a(f%(0))
c1 = calls: calls = 0
PUT (10, 10), a(f%(0))
c2 = calls
c3 = POINT(10, 10)
_DEST _CONSOLE
PRINT "GET calls"; c1; "PUT calls"; c2; "pixel"; c3
SYSTEM

FUNCTION f% (n AS INTEGER)
    calls = calls + 1
    f% = n
END FUNCTION
