' TEST: cpp
$CONSOLE:ONLY
' A FUNCTION in the index of the array of graphics GET and PUT (QB64PE-DIVERGENCES.md D-061): the position is
' held in a temporary seqN, so the FUNCTION is called once, as in QB 4.5. An array of numbers (block), of two
' dimensions, an element of an array of strings and a member of an element of a TYPE array (rest_bytes). The two
' lines with a FUNCTION in the omit color too hold the whole byte_element call in a void* seqN first, so the
' index's call comes before the later one (order.rs, Part::Bytes). The last line has no call: it keeps the old
' compiler's C++, the index written twice. Run: tests\corpus\slice\s162.
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM a(100) AS INTEGER, b(3, 3) AS INTEGER, ts(10) AS STRING, r(10) AS rec
GET (1, 2)-(3, 4), a(f%(1))
PUT (1, 2), b(f%(1), 2)
GET (1, 2)-(3, 4), ts(f%(1))
GET (1, 2)-(3, 4), r(f%(1)).y
PUT (1, 2), a(f%(1)), PSET, f%(2)
PUT (1, 2), r(f%(1)), PSET, f%(2)
PUT (1, 2), a(5)
SYSTEM

FUNCTION f% (n AS INTEGER)
    f% = n
END FUNCTION
