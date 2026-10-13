' TEST: check-fail
$CONSOLE:ONLY
' Forms the old compiler accepts and m3-get-put-bload leaves "not supported yet", one a line (design D4, D5;
' SOMEDAY.md, "Left marked by m3-get-put-bload"): VARPTR and VARSEG of anything but an array of numbers of the
' main module; SADD and VARPTR$; VARPTR in HEX$; in the array's place of graphics GET and PUT an array of strings
' or of a TYPE, a plain variable, a member, an index that calls a FUNCTION; the file form of GET and PUT. Every
' diagnostic below is a mark, none an error. A statement with a template is read after CALL since m3-sound
' (call_template_ir.bas)
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM i AS INTEGER, l AS LONG, t AS STRING, fx AS STRING * 8, one AS rec
DIM a(100) AS INTEGER, r(10) AS rec, ts(10) AS STRING, fs(10) AS STRING * 4, bt(10) AS _BIT
l = VARPTR(i)
l = VARSEG(i)
l = VARPTR(t)
l = VARPTR(fx)
l = VARPTR(r(1))
l = VARSEG(r(2))
l = VARPTR(r(2).y)
l = VARPTR(one.y)
l = VARPTR(ts(1))
l = VARSEG(fs(1))
l = VARPTR(bt(1))
l = SADD(t)
t = VARPTR$(t)
t = HEX$(VARPTR(a(1)))
t = OCT$(VARSEG(a(1)))
GET (1, 2)-(3, 4), ts
GET (1, 2)-(3, 4), fs(1)
GET (1, 2)-(3, 4), r
GET (1, 2)-(3, 4), r(2)
GET (1, 2)-(3, 4), i
GET (1, 2)-(3, 4), zz
GET (1, 2)-(3, 4), one.x
GET (1, 2)-(3, 4), a(f%(1))
PUT (1, 2), t
PUT (1, 2), bt
GET #1, , i
PUT #1, 2, i
CALL sb(a(), i)
SYSTEM

SUB sb (p() AS INTEGER, k AS INTEGER)
    DIM lc(40) AS INTEGER, z AS LONG
    STATIC st(40) AS INTEGER
    z = VARPTR(p(1))
    z = VARPTR(lc(1))
    z = VARSEG(st(1))
    z = VARPTR(k)
END SUB

FUNCTION f% (n AS INTEGER)
    f% = n
END FUNCTION
