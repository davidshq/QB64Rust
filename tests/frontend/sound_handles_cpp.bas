' TEST: cpp
$CONSOLE:ONLY
' The array of _WAVE and _SNDRAWBATCH from an element, as the old compiler writes it (m3-sound-handles design D2;
' verification\v39_x01_accepted, read with qb64pe -z): libqb's byte_element of the bytes from the checked element to
' the array's end, the index written twice. The call-site check compares the whole arrays; an element's index is
' element addressing, which it does not compare (tools\callsite\README.md)
DIM w(1 TO 64) AS _BYTE, b(1 TO 16) AS SINGLE, w2(3, 3) AS _BYTE, r AS LONG, l AS LONG
_WAVE 0, w(1), 32
_WAVE 0, w(l)
_WAVE 1, w2(1, 1), 4
_SNDRAWBATCH b(3), 2, r, 4
_SNDRAWBATCH b(l)
