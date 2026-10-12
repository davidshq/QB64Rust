$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Data"): READ into every numeric type, a string, a fixed-length string,
' an element and a member; number forms (&H, &O, &B, exponents, a quoted number, blanks); items that are no
' number; items too large for the target. An item that raises is not consumed (libqb puts the position back), so
' from "number forms" on the handler reads it as a string to step over it.
ON ERROR GOTO h
TYPE rec
    n AS LONG
    d AS DOUBLE
    f AS STRING * 4
END TYPE
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, sg AS SINGLE, db AS DOUBLE, fl AS _FLOAT
DIM o AS _OFFSET, uo AS _UNSIGNED _OFFSET, bt AS _BIT, b3 AS _BIT * 3, ub3 AS _UNSIGNED _BIT * 3
DIM s AS STRING, fx AS STRING * 4, a(3) AS LONG, sa(3) AS STRING, r AS rec

PRINT "every type from 1": RESTORE ones
READ b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, bt, b3, ub3
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; bt; b3; ub3

PRINT "2.5 and 3.5 into each": RESTORE halves
READ b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, b3, ub3
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; b3; ub3
READ b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, b3, ub3
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; b3; ub3

PRINT "-1 into each": RESTORE minus
READ b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, bt, b3, ub3
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; bt; b3; ub3

skipit = -1
PRINT "number forms into DOUBLE": RESTORE forms
FOR k = 1 TO 14: db = -7: READ db: PRINT db;: NEXT: PRINT
PRINT "number forms into LONG": RESTORE forms
FOR k = 1 TO 14: l = -7: READ l: PRINT l;: NEXT: PRINT
PRINT "number forms into _INTEGER64": RESTORE forms
FOR k = 1 TO 14: q = -7: READ q: PRINT q;: NEXT: PRINT
PRINT "number forms into STRING": RESTORE forms
FOR k = 1 TO 14: READ s: PRINT "["; s; "]";: NEXT: PRINT

PRINT "no number, each alone": RESTORE nonum
FOR k = 1 TO 8: l = -7: READ l: PRINT k; l: NEXT
PRINT "no number into DOUBLE": RESTORE nonum
FOR k = 1 TO 8: db = -7: READ db: PRINT k; db: NEXT
PRINT "no number into _INTEGER64": RESTORE nonum
FOR k = 1 TO 8: q = -7: READ q: PRINT k; q: NEXT
PRINT "no number into _UNSIGNED _INTEGER64": RESTORE nonum
FOR k = 1 TO 8: uq = 7: READ uq: PRINT k; uq: NEXT

PRINT "too large": RESTORE large
b = -7: READ b: PRINT "_BYTE 300:"; b
ub = 7: READ ub: PRINT "_UNSIGNED _BYTE 300:"; ub
ub = 7: READ ub: PRINT "_UNSIGNED _BYTE -1:"; ub
i = -7: READ i: PRINT "INTEGER 70000:"; i
ui = 7: READ ui: PRINT "_UNSIGNED INTEGER 70000:"; ui
l = -7: READ l: PRINT "LONG 5000000000:"; l
ul = 7: READ ul: PRINT "_UNSIGNED LONG 5000000000:"; ul
q = -7: READ q: PRINT "_INTEGER64 1e30:"; q
q = -7: READ q: PRINT "_INTEGER64 9223372036854775808:"; q
uq = 7: READ uq: PRINT "_UNSIGNED _INTEGER64 18446744073709551615:"; uq
uq = 7: READ uq: PRINT "_UNSIGNED _INTEGER64 18446744073709551616:"; uq
uq = 7: READ uq: PRINT "_UNSIGNED _INTEGER64 -1:"; uq
sg = -7: READ sg: PRINT "SINGLE 1e39:"; sg
db = -7: READ db: PRINT "DOUBLE 1e309:"; db
db = -7: READ db: PRINT "DOUBLE 1d309:"; db
bt = 0: READ bt: PRINT "_BIT 2:"; bt
b3 = 0: READ b3: PRINT "_BIT * 3 9:"; b3
ub3 = 0: READ ub3: PRINT "_UNSIGNED _BIT * 3 9:"; ub3
READ q: PRINT "_INTEGER64 9007199254740993:"; q
READ q: PRINT "_INTEGER64 9223372036854775807:"; q
READ uq: PRINT "_UNSIGNED _INTEGER64 9007199254740993:"; uq

skipit = 0
PRINT "strings": RESTORE strs
READ s: PRINT "["; s; "]"
READ s: PRINT "["; s; "]"
READ s: PRINT "["; s; "]"
READ s: PRINT "["; s; "]"
READ fx: PRINT "["; fx; "]"
READ fx: PRINT "["; fx; "]"
READ fx: PRINT "["; fx; "]"

PRINT "element, member": RESTORE places
k = 2
READ a(k), sa(k + 1), r.n, r.d, r.f
PRINT a(2); "["; sa(3); "]"; r.n; r.d; "["; r.f; "]"
SYSTEM

ones:
DATA 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1
halves:
DATA 2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5
DATA 3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5,3.5
minus:
DATA -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1
forms:
DATA &H10, &O17, &B101, 1e3, 1d3, 1.5E+2, "12", " 12 ", +5, .5, -.5, 1f2, &HFFFF, &HFFFFFFFF
nonum:
DATA x, "abc", 12abc, , "", 1 2, 1e, --1
large:
DATA 300, 300, -1, 70000, 70000, 5000000000, 5000000000, 1e30, 9223372036854775808
DATA 18446744073709551615, 18446744073709551616, -1, 1e39, 1e309, 1d309, 2, 9, 9
DATA 9007199254740993, 9223372036854775807, 9007199254740993
strs:
DATA bare  word, "  quoted, with comma  ", , ""
DATA ab, abcdefgh, "a b c d"
places:
DATA 5, elem, 6, 7.5, member
h:
PRINT "  error"; ERR
IF skipit <> 0 AND ERR <> 4 THEN READ junk$
RESUME NEXT
