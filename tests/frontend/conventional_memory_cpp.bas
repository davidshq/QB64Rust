' TEST: cpp
$CONSOLE:ONLY
' An array in conventional memory, as the old compiler writes it (m3-get-put-bload design D3 to D5; study\00 §5):
' a static one (a) takes its elements from cmem_static_pointer, rounded up to 16 bytes, with flags 1+2+4; a
' dynamic one (dy) starts with flag 4, and every allocation and release of a dynamic array of numbers tests that
' flag, since the array may be a parameter. b is not named in VARPTR or VARSEG and is allocated as before.
' VARPTR is the checked position times the element's size, VARSEG the array's distance from the start of the
' block in segments. The array of GET and PUT is libqb's byte_element with a temporary declared with the
' variables of its body
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER, m(1 TO 3, 1 TO 4) AS LONG, n AS INTEGER, l AS LONG
n = 8
DIM dy(1 TO n) AS LONG
DEF SEG = VARSEG(a(1))
POKE VARPTR(a(n)), 65
l = VARPTR(m(2, 3)) + VARSEG(dy(1))
ERASE dy
REDIM _PRESERVE dy(1 TO 4) AS LONG
GET (1, 2)-(3, 4), b
PUT (1, 2), m(2, n), XOR
CALL sb(b())
SYSTEM

SUB sb (p() AS INTEGER)
    PUT (1, 2), p
END SUB
