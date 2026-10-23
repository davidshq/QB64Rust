' TEST: cpp
$CONSOLE:ONLY
' The array's place of graphics GET and PUT that is no array of numbers, as the old compiler writes it
' (m4-memory-records design D5; its C++ of verification\v56_d): a number's bytes, a string's characters, a
' fixed-length string's characters, from a member to the end of its variable, an undeclared name (an implicit
' SINGLE) and a name alone that is an array of strings (the SINGLE of that name), an element of an array of
' strings (its characters; sa() the first), from an element or a member of an array of fixed-length strings or
' of a TYPE to the array's end, and x() of a name that is no array (the implicit array)
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM i AS INTEGER, s AS STRING, fx AS STRING * 8, r AS rec, q AS _INTEGER64
DIM sa(3) AS STRING, fa(1 TO 3) AS STRING * 4, ra(1 TO 3) AS rec
GET (0, 0)-(1, 1), i
GET (0, 0)-(1, 1), s
GET (0, 0)-(1, 1), fx
GET (0, 0)-(1, 1), r
PUT (4, 4), r.y, PSET
GET (0, 0)-(1, 1), zz
GET (0, 0)-(1, 1), sa
GET (0, 0)-(1, 1), sa(q)
PUT (4, 4), sa()
GET (0, 0)-(1, 1), fa(q)
GET (0, 0)-(1, 1), fa()
PUT (4, 4), ra(q), XOR
GET (0, 0)-(1, 1), ra
GET (0, 0)-(1, 1), ra(q).y
GET (0, 0)-(1, 1), w%()
SYSTEM
