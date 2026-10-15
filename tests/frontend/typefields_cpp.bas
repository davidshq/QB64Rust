' TEST: cpp
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Dynamic member arrays in the emitter (m3-typefields tasks 3.1 and 3.3, design D3, D4, D6, D9): the aligned
' layout of a type that owns one (verification\v35_x52: t is 24 bytes with b at 16, o is 40 bytes with i at 8 and
' d at 32), the functions of each owner type and of each dynamic member array in global.txt, a variable and a
' local made and released with their members, REDIM, ERASE, LBOUND, an element read and stored through the guard,
' the member as an array argument, and a whole-value copy that clones the descriptor.
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
DIM x AS t, y AS t, z AS o
x.b = 1: z.d = 2: z.i.b = 3
REDIM _PRESERVE x.v(1 TO 5) AS LONG
x.v(2) = 5
PRINT LBOUND(x.v); x.v(2)
fill x.v()
y = x
ERASE x.v
loc
SYSTEM

SUB fill (a() AS LONG)
    a(LBOUND(a)) = 1
END SUB

SUB loc
    DIM w AS t
    PRINT UBOUND(w.v)
END SUB
