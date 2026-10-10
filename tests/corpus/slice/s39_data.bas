$CONSOLE:ONLY
' DATA, READ and RESTORE (spec language/data-read): the program's data in file order (main module, a block that
' never runs, after SYSTEM, in a SUB and a FUNCTION), bare DATA, quoted and unquoted items; READ into every numeric
' type, strings, fixed-length strings, elements and members; number forms; items that are no number (error 2) or
' too large (error 6) are not taken, and the later targets of that READ get 0; out of data (error 4); RESTORE to
' the start and to labels of any body.
ON ERROR GOTO h
TYPE rec
    n AS LONG
    d AS DOUBLE
    f AS STRING * 4
END TYPE
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, sg AS SINGLE, db AS DOUBLE, fl AS _FLOAT
DIM o AS _OFFSET, uo AS _UNSIGNED _OFFSET, bt AS _BIT, b3 AS _BIT * 3, ub3 AS _UNSIGNED _BIT * 3
DIM s AS STRING, t AS STRING, fx AS STRING * 4, a(3) AS LONG, sa(3) AS STRING, r AS rec, k AS LONG
DIM SHARED skipit AS LONG

first:
DATA m1, "m 2"
IF 0 THEN
    DATA in_if
END IF
PRINT "the order": DATA after_colon
DATA
DATA "q", , last
FOR k = 1 TO 9
    READ s
    PRINT k; "["; s; "]"
NEXT

PRINT "every type from 1": RESTORE ones
READ b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, b3, ub3
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; b3; ub3
PRINT "2.5 into each": RESTORE halves
READ b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, b3, ub3
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; b3; ub3
PRINT "-1 into a _BIT and into the signed types": RESTORE minus
READ bt, b, i, l, q, sg, db, fl, o, b3
PRINT bt; b; i; l; q; sg; db; fl; o; b3

skipit = -1
PRINT "number forms into DOUBLE": RESTORE forms
FOR k = 1 TO 14: db = -7: READ db: PRINT db;: NEXT: PRINT
PRINT "number forms into LONG": RESTORE forms
FOR k = 1 TO 14: l = -7: READ l: PRINT l;: NEXT: PRINT
PRINT "number forms into _INTEGER64": RESTORE forms
FOR k = 1 TO 14: q = -7: READ q: PRINT q;: NEXT: PRINT
PRINT "number forms into _UNSIGNED _INTEGER64": RESTORE forms
FOR k = 1 TO 14: uq = 7: READ uq: PRINT uq;: NEXT: PRINT
PRINT "number forms into STRING": RESTORE forms
FOR k = 1 TO 14: READ s: PRINT "["; s; "]";: NEXT: PRINT
PRINT "no number": RESTORE nonum
FOR k = 1 TO 8: l = -7: READ l: PRINT k; l: NEXT
PRINT "too large": RESTORE large
b = -7: READ b: PRINT "_BYTE 300:"; b
ub = 7: READ ub: PRINT "_UNSIGNED _BYTE -1:"; ub
i = -7: READ i: PRINT "INTEGER 70000:"; i
l = -7: READ l: PRINT "LONG 5000000000:"; l
q = -7: READ q: PRINT "_INTEGER64 9223372036854775808:"; q
uq = 7: READ uq: PRINT "_UNSIGNED _INTEGER64 18446744073709551615:"; uq
uq = 7: READ uq: PRINT "_UNSIGNED _INTEGER64 18446744073709551616:"; uq
sg = -7: READ sg: PRINT "SINGLE 1e39:"; sg
db = -7: READ db: PRINT "DOUBLE 1e309:"; db
bt = 0: READ bt: PRINT "_BIT 2:"; bt
ub3 = 0: READ ub3: PRINT "_UNSIGNED _BIT * 3 9:"; ub3
READ q: PRINT "_INTEGER64 9007199254740993:"; q
skipit = 0

PRINT "strings": RESTORE strs
READ s: PRINT "["; s; "]"
READ s: PRINT "["; s; "]"
READ s, t: PRINT "["; s; "]["; t; "]"
READ fx: PRINT "["; fx; "]"
READ fx: PRINT "["; fx; "]"
PRINT "element, member": RESTORE places
k = 2
READ a(k), sa(k + 1), r.n, r.d, r.f
PRINT a(2); "["; sa(3); "]"; r.n; r.d; "["; r.f; "]"

PRINT "an item too large in the middle: it stays": RESTORE toolarge
l = -7: b = -7: i = -7
READ l, b, i
PRINT l; b; i
READ i: PRINT "next item:"; i
PRINT "no number in the middle: it stays": RESTORE notnum
l = -7: i = -7: q = -7: s = "old"
READ l, i, q, s
PRINT l; i; q; "["; s; "]"
READ s: PRINT "next item: "; s
PRINT "an element out of range in the middle": RESTORE places
l = -7: i = -7: k = 9
READ l, a(k), i
PRINT l; i
READ s: PRINT "next item: "; s

PRINT "out of data": RESTORE tail
READ l: PRINT l
l = -7: i = -7: s = "old": t = "old"
READ l, i, s
PRINT l; i; "["; s; "]"
READ s: PRINT "["; s; "]"
READ i, t: PRINT i; "["; t; "]"
PRINT "RESTORE, then the first item": RESTORE: READ s: PRINT s
PRINT "RESTORE to a label on a DATA line": RESTORE strs2: READ s: PRINT s
PRINT "RESTORE to a label of a SUB": RESTORE insub: READ s: PRINT s
PRINT "RESTORE to a label with no DATA after it": RESTORE nothing: s = "old": READ s: PRINT "["; s; "]"
PRINT "RESTORE FIRST": RESTORE FIRST: READ s: PRINT s
p
READ s: PRINT "back in main: "; s
PRINT f&
SYSTEM
DATA after_system
h:
PRINT "  error"; ERR
IF skipit <> 0 AND ERR <> 4 THEN READ junk$
RESUME NEXT

ones:
DATA 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1
halves:
DATA 2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5
minus:
DATA -1,-1,-1,-1,-1,-1,-1,-1,-1,-1
forms:
DATA &H10, &O17, &B101, 1e3, 1d3, 1.5E+2, "12", " 12 ", +5, .5, -.5, 1f2, &HFFFF, &HFFFFFFFF
nonum:
DATA x, "abc", 12abc, , "", 1 2, 1e, --1
large:
DATA 300, -1, 70000, 5000000000, 9223372036854775808
DATA 18446744073709551615, 18446744073709551616, 1e39, 1e309, 2, 9, 9007199254740993
strs:
DATA bare  word, "  quoted, with comma  ", , ""
strs2: DATA ab, abcdefgh, "unclosed, to the end
places:
DATA 5, elem, 6, 7.5, member
toolarge:
DATA 10, 300, 30, 40
notnum:
DATA 11, zz, 33, 44

SUB p
    DIM s AS STRING
    PRINT "in p: RESTORE to a main-module label": RESTORE strs: READ s: PRINT s
    insub:
    DATA in_sub
    PRINT "in p: RESTORE to its own label": RESTORE insub: READ s: PRINT s
    PRINT "in p: RESTORE": RESTORE: READ s: PRINT s
END SUB

FUNCTION f&
    DATA 77
    tail:
    DATA 51, 52
    nothing:
    RESTORE insub
    READ s$, l&: f& = l&
END FUNCTION
