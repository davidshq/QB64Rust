' TEST: cpp
' The array of _UCHARPOS as the old compiler writes it (m3-images-fonts-mouse, design D1; qb64pe.bas 20903):
' libqb's byte_element from the element given (or the first, for p()) to the array's end, with a temporary of its
' own, as for the array of a graphics GET. The element forms are not in tests\callsite, which does not compare an
' element's index (tools\callsite\README.md); the screen program f02_loaded_fonts runs them
DIM l AS LONG, i AS INTEGER, t AS STRING, p(10) AS LONG, z(3, 3) AS LONG
l = _UCHARPOS(t, p())
l = _UCHARPOS(t, p(i))
l = _UCHARPOS(t, z(1, 2), 8)
