$CONSOLE
$SCREENHIDE
' Verification (m2-parser-breadth, M5): PUT and GET in graphics and file forms, including file forms without #.
img& = _NEWIMAGE(20, 20, 256)
_DEST img&: _SOURCE img&
DIM spr(100) AS INTEGER
PSET (1, 1), 5
GET (0, 0)-(3, 3), spr()
CLS
PUT (10, 10), spr(), PSET
g1 = POINT(11, 11)
_DEST _CONSOLE
OPEN "v16_m5_put_forms.tmp" FOR BINARY AS #1
x& = 42: PUT #1, , x&
y& = 43: PUT 1, , y&
z& = 0: GET #1, 1, z&
w& = 0: GET 1, 5, w&
CLOSE #1
KILL "v16_m5_put_forms.tmp"
PRINT "graphics PUT read back"; g1
PRINT "file PUT/GET"; z&; w&
SYSTEM
