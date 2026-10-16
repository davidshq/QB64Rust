' TEST: ir
' Graphics GET and PUT (m3-get-put-bload design D5; verification\v28_x01): every part of the two templates, STEP
' before either point, the five action words, _CLIP, a colour, parts left out. The array is a slot of its own
' kind: the bytes of an array, whole (a, a()) or from an element (a(5), m(2, 3)); static, dynamic, of a procedure
' (lc), STATIC there (st) or a parameter (p). The coordinates are passed as they are
DIM a(100) AS INTEGER, m(1 TO 10, 1 TO 20) AS LONG, c(100) AS SINGLE
DIM i AS INTEGER, l AS LONG, s AS SINGLE, d AS DOUBLE
REDIM rd(1 TO 30) AS LONG
SCREEN 12
GET (1, 2)-(3, 4), a
GET (1, 2)-(3, 4), a()
GET (i, l)-(s, d), a(5)
GET STEP(1, 2)-(3, 4), m(2, i + 1)
GET (1, 2)-STEP(3, 4), c
GET STEP(1, 2)-STEP(3, 4), rd(5)
PUT (1, 2), a
PUT (i, l), a(), PSET
PUT (s, d), m, PRESET
PUT (1, 2), c, AND
PUT (1, 2), rd, OR
PUT (1.5, 2.5), a(i), XOR
PUT STEP(1, 2), a
PUT (1, 2), a, _CLIP
PUT (1, 2), a, _CLIP PSET, 5
PUT (1, 2), a, _CLIP , l
PUT (1, 2), a, , 5
PUT (1, 2), a,
CALL sb(a())
SYSTEM

SUB sb (p() AS INTEGER)
    DIM lc(40) AS INTEGER
    STATIC st(40) AS INTEGER
    GET (1, 2)-(3, 4), p
    GET (1, 2)-(3, 4), lc
    PUT (1, 2), st(3)
    PUT (1, 2), p(1), PSET
END SUB
