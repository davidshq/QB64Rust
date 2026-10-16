' TEST: check-fail
$CONSOLE:ONLY
' Forms of TAB, SPC, graphics GET and PUT, DEF SEG, BLOAD, BSAVE, PEEK, POKE, VARPTR, VARSEG, _RGB32, _RGBA32 and
' _FULLSCREEN the old compiler rejects, one a line (verification\v28_x03-x11, x14-x19, x21, x22, x24-x52, x54,
' x61, x64, x93, x94): each is an error here too, none a "not supported yet". Two more that it rejects keep a
' mark here by older rules: TAB(5) = 1 (x53: a built-in function's name as a place) and CALL DEF SEG (x67)
CONST k = 5
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, t AS STRING, u AS _UNSIGNED LONG
DIM a(100) AS INTEGER, b(1 TO 10, 1 TO 20) AS LONG, x AS INTEGER
PRINT TAB(5) i
PRINT TAB(3) TAB(9); "t"
PRINT TAB("a"); 1
PRINT TAB(); 1
PRINT TAB(1, 2); 1
PRINT TAB; 1
PRINT SPC(t); 1
GET (1, 2)-(3, 4)
GET (1, 2), a
GET (1, 2)-(3, 4), i + 1
GET (1, 2)-(3, 4), 5
GET (1, 2)-(3, 4), a, PSET
PUT (1, 2)
PUT (1, 2)-(3, 4), a
PUT (1, 2), a, FOO
PUT (1, 2), a, PSET XOR
PUT (1, 2), a, PSET _CLIP
PUT (1, 2), STEP a
GET ("a", 2)-(3, 4), a
DEF SEG 5
DEF SEG = "a"
DEF SEG =
DEF SEG = 1, 2
BLOAD
BLOAD 5
BLOAD "f", "g"
BLOAD "f", 1, 2
BSAVE "f", 1
BSAVE 5, 1, 2
l = VARPTR(5)
l = VARPTR(i + 1)
l = VARSEG()
l = VARPTR(i, l)
l = VARSEG("a")
l = VARPTR(ABS(i))
l = PEEK("a")
POKE 1
l = PEEK(1, 2)
u = _RGB32()
u = _RGB32(1, 2, 3, 4, 5)
u = _RGBA32(1, 2, 3)
u = _RGB32("a")
_FULLSCREEN 5
_FULLSCREEN _SMOOTH
_FULLSCREEN _OFF _STRETCH
VARPTR(i)
DEF
PUT (1, 2), k
GET (1), , x
PUT (i), 2, x
