$CONSOLE
$SCREENHIDE
' Verification (m4-memory-records, task 1.1): graphics GET and PUT with a plain variable, a string, a
' fixed-length string, a TYPE value and member, an undeclared name, and arrays of strings, fixed-length strings
' and a TYPE, whole or from an element. Prints the bytes each GET took and the POINTs each PUT drew; a value too
' small raises error 5.
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM SHARED img AS LONG
DIM d AS DOUBLE, t AS STRING, fx AS STRING * 8, r AS rec, q AS _INTEGER64, k AS INTEGER
DIM sa(3) AS STRING, fa(1 TO 3) AS STRING * 4, ra(1 TO 3) AS rec, r1 AS rec
img = _NEWIMAGE(8, 8, 256)
_DEST img: _SOURCE img
PSET (0, 0), 1: PSET (1, 0), 2: PSET (0, 1), 3: PSET (1, 1), 4
ON ERROR GOTO h
GET (0, 0)-(1, 1), d
say "d " + bytes$(MKD$(d))
GET (0, 0)-(1, 1), q
say "q " + bytes$(_MK$(_INTEGER64, q))
t = SPACE$(9)
GET (0, 0)-(1, 1), t
say "t " + bytes$(t)
GET (0, 0)-(1, 1), fx
say "fx " + bytes$(fx)
GET (0, 0)-(0, 1), r
say "r" + STR$(r.x) + STR$(r.y)
GET (0, 0)-(0, 0), r1.y
say "r1.y" + STR$(r1.x) + STR$(r1.y)
GET (0, 0)-(1, 1), zz#
say "zz# " + bytes$(MKD$(zz#))
sa(2) = SPACE$(8)
GET (0, 0)-(1, 1), sa(2)
say "sa(2) " + bytes$(sa(2))
sa(0) = SPACE$(8)
GET (0, 0)-(1, 1), sa()
say "sa() " + bytes$(sa(0))
GET (0, 0)-(1, 1), fa(2)
say "fa(2) " + bytes$(fa(2)) + bytes$(fa(3))
GET (0, 0)-(1, 1), fa()
say "fa() " + bytes$(fa(1)) + bytes$(fa(2))
GET (0, 0)-(1, 1), ra(2)
say "ra(2)" + STR$(ra(2).x) + STR$(ra(2).y) + STR$(ra(3).x) + STR$(ra(3).y)
GET (0, 0)-(1, 1), ra(1).y
say "ra(1).y" + STR$(ra(1).y) + STR$(ra(2).x) + STR$(ra(2).y)
GET (0, 0)-(1, 1), ra()
say "ra()" + STR$(ra(1).x) + STR$(ra(1).y) + STR$(ra(2).x)
' Too small: error 5.
GET (0, 0)-(1, 1), k
GET (0, 0)-(1, 1), r
GET (0, 0)-(1, 1), sa(1)
GET (0, 0)-(1, 1), fa(3)
GET (0, 0)-(1, 1), small%
' PUT each back.
CLS: PUT (4, 4), d: pts "d"
CLS: PUT (4, 4), q: pts "q"
CLS: PUT (4, 4), t: pts "t"
CLS: PUT (4, 4), fx: pts "fx"
CLS: PUT (4, 4), r, PSET: pts "r"
CLS: PUT (4, 4), zz#: pts "zz#"
CLS: PUT (4, 4), sa(2): pts "sa(2)"
CLS: PUT (4, 4), sa(): pts "sa()"
CLS: PUT (4, 4), fa(2), PSET: pts "fa(2)"
CLS: PUT (4, 4), fa(): pts "fa()"
CLS: PUT (4, 4), ra(2): pts "ra(2)"
CLS: PUT (4, 4), ra(): pts "ra()"
CLS: PUT (4, 4), r1.y: pts "r1.y"
CLS: PUT (4, 4), never%: pts "never%"
CLS: PUT (4, 4), sa(1): pts "sa(1)"
SYSTEM
h:
say "error" + STR$(ERR) + " at line" + STR$(_ERRORLINE)
RESUME NEXT

SUB say (s AS STRING)
    _DEST _CONSOLE
    PRINT s
    _DEST img
END SUB

SUB pts (n AS STRING)
    say n + STR$(POINT(4, 4)) + STR$(POINT(5, 4)) + STR$(POINT(4, 5)) + STR$(POINT(5, 5))
END SUB

FUNCTION bytes$ (s AS STRING)
    DIM o AS STRING
    FOR i = 1 TO LEN(s): o = o + STR$(ASC(s, i)): NEXT
    bytes$ = o
END FUNCTION
