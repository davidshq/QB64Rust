' TEST: cpp
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Dynamic member arrays in the emitter (m3-typefields tasks 3.1 and 3.3, design D3, D4, D6, D9): the aligned
' layout of a type that owns one (verification\v35_x52: t is 24 bytes with b at 16, o is 40 bytes with i at 8 and
' d at 32), the functions of each owner type and of each dynamic member array in global.txt, a variable and a
' local made and released with their members, REDIM, ERASE, LBOUND, an element read and stored through the guard,
' the member as an array argument, and a whole-value copy that clones the descriptor. Task 4.3 (design D7,
' verification\v35_x56, x61): the copies find their places once per statement, as the old compiler's do: a type
' with no string (its parts, then the descriptors, also those of a nested owner), and one with a string (each part
' in order, a dynamic member array erased and then set from the source). Task 5.1 (design D8): PUT and GET of a
' whole array of an owner type through its file functions (its declared layout, the bounds checked, the buffer
' unpacked after a GET that raised nothing), only for the type a file statement names. Task 5.2: _MEM of a whole
' dynamic member array and of an element of one, with the descriptor's lock (verification\v35_x66), also when an
' index of the value the member is part of reads another dynamic member (the lock is still the element's own).
TYPE t
    a AS INTEGER
    v(2) _DYNAMIC AS LONG
    b AS INTEGER
END TYPE
TYPE o
    c AS _BYTE
    i AS t
    d AS _BYTE
END TYPE
TYPE s
    n AS LONG
    w AS STRING
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t, y AS t, z AS o, z2 AS o
REDIM p(1) AS s
x.b = 1: z.d = 2: z.i.b = 3
REDIM _PRESERVE x.v(1 TO 5) AS LONG
x.v(2) = 5
PRINT LBOUND(x.v); x.v(2)
fill x.v()
y = x
z2 = z
p(1) = p(0)
ERASE x.v
REDIM r(1 TO 2) AS o
OPEN "typefields_cpp.tmp" FOR BINARY AS #1
PUT #1, 1, r()
GET #1, , r()
CLOSE #1
DIM m AS _MEM
m = _MEM(x.v())
m = _MEMELEMENT(r(1).i.v(x.v(1)))
m = _MEM(r(x.v(1)).i.v(1))
loc
SYSTEM

SUB fill (a() AS LONG)
    a(LBOUND(a)) = 1
END SUB

SUB loc
    DIM w AS t
    PRINT UBOUND(w.v)
END SUB
