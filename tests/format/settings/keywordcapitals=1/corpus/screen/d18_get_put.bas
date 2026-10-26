' Graphics GET and PUT in SCREEN 12: each action word over a background, no action, a colour, _CLIP at two edges,
' STEP; the array whole and from an element, of two dimensions, of LONG, dynamic, and in a SUB as a parameter, a
' local and a STATIC array; a PUT that leaves the screen and a GET into an array that is too small raise error 5.
DEFINT A-Z
DIM SHARED errs
DIM a(150), m(1 TO 4, 1 TO 40), l(80) AS LONG, small(3)
DIM n
SCREEN 12
ON ERROR GOTO handler
' The picture: a filled box with two diagonals and a dot.
LINE (10, 10)-(30, 26), 4, BF
LINE (10, 10)-(30, 26), 14
LINE (10, 26)-(30, 10), 10
PSET (20, 18), 15
GET (10, 10)-(30, 26), a()
' A background for the actions.
LINE (0, 60)-(639, 100), 3, BF
PUT (50, 10), a()
PUT (50, 70), a()
PUT (100, 70), a(), PSET
PUT (150, 70), a(), PRESET
PUT (200, 70), a(), AND
PUT (250, 70), a(), OR
PUT (300, 70), a(), XOR
PUT (350, 70), a(), , 4
PUT (400, 70), a(), PSET , 14
PUT (450, 70), a(), _CLIP, 10
PUT (630, 70), a(), _CLIP PSET
PUT (-10, 200), a(), _CLIP
PSET (100, 150), 0
PUT STEP(10, 10), a(), PSET
' From an element; two dimensions; LONG; dynamic arrays.
GET (10, 10)-(20, 18), a(60)
PUT (150, 150), a(60), PSET
GET (10, 10)-(30, 26), m()
PUT (200, 150), m(), PSET
GET (10, 10)-(30, 26), m(1, 2)
PUT (230, 150), m(1, 2), PSET
GET (10, 10)-(30, 26), l()
PUT (260, 150), l(), PSET
n = 150
DIM dy(n)
GET (10, 10)-(30, 26), dy()
PUT (300, 150), dy(), PSET
REDIM rd(1 TO 150)
PSET (10, 10), 0
GET STEP(0, 0)-STEP(20, 16), rd(n - 149)
PUT (350, 150), rd(1), OR
' In a SUB.
CALL stamp(a(), 400, 150)
' Errors.
PUT (630, 300), a(), PSET
GET (10, 10)-(30, 26), small()
PUT (10, 300), small(), PSET
GET (10, 10)-(700, 26), a()
LOCATE 25, 1: PRINT "errors"; errs
SYSTEM
handler:
errs = errs + 1
LOCATE 20 + errs, 1: PRINT "error"; ERR
RESUME NEXT

SUB stamp (p(), x, y)
    DIM lc(150)
    STATIC st(150)
    PUT (x, y), p(), PSET
    GET (x, y)-(x + 20, y + 16), lc()
    PUT (x + 50, y), lc(), PSET
    GET (10, 10)-(30, 26), st(2)
    PUT (x + 100, y), st(2), XOR
    PUT (x + 150, y), p(), PRESET
END SUB
