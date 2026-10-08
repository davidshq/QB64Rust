$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.2): passing by reference across signedness (study\02 section 1.7:
' the same storage with a pointer cast), and _BIT variables passed to integer parameters.
ON ERROR GOTO h
DIM sb AS _BYTE, ub AS _UNSIGNED _BYTE, si AS INTEGER, ui AS _UNSIGNED INTEGER
DIM sl AS LONG, ul AS _UNSIGNED LONG, sq AS _INTEGER64, uq AS _UNSIGNED _INTEGER64
DIM so AS _OFFSET, uo AS _UNSIGNED _OFFSET
sb = -1: showub sb: PRINT " after:"; sb
ub = 255: showsb ub: PRINT " after:"; ub
si = -1: showui si: PRINT " after:"; si
sl = -1: showul sl: PRINT " after:"; sl
ul = 4294967295: showsl ul: PRINT " after:"; ul
sq = -1: showuq sq: PRINT " after:"; sq
so = -1: showuo so: PRINT " after:"; so
sq = -1: showuo sq: PRINT " after:"; sq
so = -1: showsq so: PRINT " after:"; so
uq = 5: showuo uq: PRINT " after:"; uq
sl = -1: showsq sl: PRINT " after:"; sl
sq = -1: showso sq: PRINT " after:"; sq
uo = 18446744073709551615~&&: showsq uo: PRINT " after:"; uo
uo = 18446744073709551615~&&: showuq uo: PRINT " after:"; uo
so = -1: showuq so: PRINT " after:"; so
' _BIT variables passed to integer parameters (a _BIT parameter fails the C++ build: v21_x20-x24)
DIM b5 AS _BIT * 5, u5 AS _UNSIGNED _BIT * 5, b40 AS _BIT * 40
b5 = -16: showsl b5: PRINT " after:"; b5
b5 = 7: showsi b5: PRINT " after:"; b5
u5 = 31: showul u5: PRINT " after:"; u5
u5 = 31: showsl u5: PRINT " after:"; u5
b40 = -1: showsq b40: PRINT " after:"; b40
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

SUB showsb (x AS _BYTE)
PRINT "_BYTE param:"; x;: x = -2
END SUB
SUB showub (x AS _UNSIGNED _BYTE)
PRINT "_UNSIGNED _BYTE param:"; x;: x = 254
END SUB
SUB showsi (x AS INTEGER)
PRINT "INTEGER param:"; x;: x = 99
END SUB
SUB showui (x AS _UNSIGNED INTEGER)
PRINT "_UNSIGNED INTEGER param:"; x;: x = 65534
END SUB
SUB showsl (x AS LONG)
PRINT "LONG param:"; x;: x = 5
END SUB
SUB showul (x AS _UNSIGNED LONG)
PRINT "_UNSIGNED LONG param:"; x;: x = 5
END SUB
SUB showsq (x AS _INTEGER64)
PRINT "_INTEGER64 param:"; x;: x = 6
END SUB
SUB showuq (x AS _UNSIGNED _INTEGER64)
PRINT "_UNSIGNED _INTEGER64 param:"; x;: x = 6
END SUB
SUB showso (x AS _OFFSET)
PRINT "_OFFSET param:"; x;: x = 8
END SUB
SUB showuo (x AS _UNSIGNED _OFFSET)
PRINT "_UNSIGNED _OFFSET param:"; x;: x = 7
END SUB
