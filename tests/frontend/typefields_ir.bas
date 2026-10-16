' TEST: ir
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Dynamic member arrays in the IR (m3-typefields task 3.2, design D5): REDIM of one is an Allocate of the member
' array, with its mode and as many pairs of bounds as written (another number of dimensions than declared
' included), its place found once; ERASE of one is a Release; a whole-value copy of an owner value is one
' CopyWhole, as for any user type.
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM a(3) AS t
DIM i AS LONG, p AS t, q AS t
REDIM _RETAIN a(i).v(2 TO 5) AS LONG
REDIM p.v(1 TO 2, 3 TO 4) AS LONG
ERASE p.v
q = p
PRINT a(i).v(3); UBOUND(p.v, 2)
