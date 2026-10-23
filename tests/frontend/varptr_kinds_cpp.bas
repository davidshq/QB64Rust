' TEST: cpp
$CONSOLE:ONLY
' Variables in conventional memory, as the old compiler writes them (m4-memory-records design D1-D3; its C++ of
' verification\v56_a1, a3, a4): a scalar named by VARPTR takes its bytes from the top of dblock in the order of
' declaration (n, which no VARPTR names, stays where it was); every _BIT scalar of the main module lies there
' too; a string's descriptor is made by qbs_new_cmem, a fixed-length string's bytes and descriptor are taken
' there, an array of strings makes each element's descriptor there, and a dynamic one tests flag 4 when it is
' allocated. VARPTR of a variable is its offset from dblock, of a string its descriptor's, of a member its
' variable's plus the member's; VARSEG of a variable 80; SADD the offset of the characters; VARPTR$ the type
' byte and the offset
TYPE t
    a AS LONG
    v AS _BYTE
END TYPE
DIM i AS INTEGER, n AS LONG, bt AS _BIT, s AS STRING, fx AS STRING * 4, r AS t, l AS _INTEGER64, w AS STRING
DIM sa(1 TO 3) AS STRING
REDIM ds(1 TO 2) AS STRING
l = VARPTR(i) + VARSEG(i) + VARPTR(s) + VARPTR(fx) + VARPTR(r.v) + VARSEG(r)
l = VARPTR(sa(2)) + VARSEG(ds(1)) + SADD(s)
w = VARPTR$(i) + VARPTR$(fx) + VARPTR$(r.v)
REDIM _PRESERVE ds(1 TO 4) AS STRING
SYSTEM
