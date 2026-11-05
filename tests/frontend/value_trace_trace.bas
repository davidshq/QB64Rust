' TEST: trace
' A value-trace build: a record call where each store is made. A variable and a member with no index are read
' after the store; an element under the store's guard, at tmp_long; a place through a member with an index under
' mbr_ok. A built-in that is given a place holds what it was given in a temporary and reads through it after the
' call. A variable passed by reference is read after the statement, before a jump the statement takes; an array
' passed gets an alias counter. A copy of a whole type records its member array in a loop. A copy of a whole
' array, _ARRAYCOPY and GET # into an array record the elements they wrote in a loop; _UCHARPOS as many as it
' returns. FIELD and GET # into the record buffer read the FIELD's strings again. REDIM and a store at an address
' get a counter before the statement. The tables and the run-time part follow the program's globals.
$CONSOLE:ONLY
TYPE pt
    x AS INTEGER
    v(2) AS LONG
END TYPE
DIM p AS pt, q AS pt, ships(3) AS pt
DIM a(5) AS LONG, c(5) AS LONG, n$(2)
k = 1
a(k) = 2
p.x = 3
ships(k).x = 4
p.v(k) = 5
q = p
SWAP a(1), a(2)
MID$(n$(1), 1, 1) = "x"
INPUT k, n$(2)
READ n$(0)
DATA abc
CALL bump(k)
fill a()
IF twice(k) THEN PRINT k
c() = a()
_ARRAYCOPY a(1 TO 2) TO c(4)
k = _UCHARPOS("ab", c())
OPEN "r.dat" FOR RANDOM AS #1 LEN = 4
FIELD #1, 4 AS f$
GET #1, 1
GET #1, 1, c()
REDIM d(3)
POKE 100, 1
SYSTEM

SUB bump (n)
    n = n + 1
END SUB

SUB fill (b() AS LONG)
    b(1) = 7
END SUB

FUNCTION twice (n)
    n = n * 2
    twice = n
END FUNCTION
