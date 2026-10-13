$CONSOLE:ONLY
' Slice (m3-console-files, task 6.2; from verification\v30_j_field, v30_k_field_errors, v30_l_lset_rset): FIELD,
' LSET and RSET, 20 numbered lines. Records written through fields and read back with GET and PUT with no
' variable; what a field string holds after FIELD; a second FIELD on the file (both name the buffer from its first
' byte); = to a field variable (the string changes, the buffer does not, and the string is still a field); GET and
' PUT with a variable beside fields (the buffer is not theirs); a width of 0; after CLOSE; the errors of FIELD
' (widths beyond the record 50, a negative width 5, not open 52, not random 54) and of GET and PUT with no variable
' (record 0: 63, a binary file: 54, ACCESS READ and WRITE: 75); the file number before a target's index before
' its width; elements and members as fields. LSET and RSET on any string: shorter, equal and longer values, an
' empty target, fixed-length strings, elements, members, the target in its own value, an error in the value.
' The files are s86_field_lset.tmp and s86_field_lset_2.tmp and are removed.
ON ERROR GOTO handler
TYPE t
    s AS STRING * 6
    v AS STRING
    n AS LONG
END TYPE
DIM f AS STRING, a AS STRING, b AS STRING, c AS STRING, d AS STRING, w AS STRING, k AS LONG, l AS LONG
DIM ub AS _UNSIGNED _BYTE, x AS STRING * 8, y AS STRING * 5, e(1 TO 3) AS STRING, r AS t, q(2) AS t
DIM fx(2) AS STRING * 4, s AS SINGLE
f = "s86_field_lset.tmp"
OPEN f FOR RANDOM AS #1 LEN = 8
a = "before"
FIELD #1, 3 AS a, 5 AS b
PRINT "1:"; LEN(a); LEN(b); ASC(a); ASC(b)
LSET a = "xy": LSET b = "hello"
PRINT "2: ["; a; "]["; b; "]";: PUT #1, 1: PRINT LOF(1); SEEK(1)
LSET a = "": LSET b = ""
PRINT "3: ["; a; "]["; b; "]";: GET #1, 1: PRINT "["; a; "]["; b; "]"; SEEK(1)
RSET a = "z": RSET b = "ab"
PRINT "4: ["; a; "]["; b; "]";: PUT #1: PRINT LOF(1); SEEK(1);: PUT #1, 4: PRINT LOF(1); SEEK(1)
PRINT "5:";: GET #1, 2: PRINT " ["; a; "]["; b; "]";: GET #1, 3: PRINT ASC(a); ASC(b); EOF(1);: GET #1, 9: PRINT ASC(a); EOF(1); LOF(1)
GET #1, 1
FIELD #1, 8 AS w
PRINT "6: ["; w; "]["; a; "]["; b; "]";: LSET w = "12345678": PRINT "["; a; "]["; b; "]";: LSET b = "B": PRINT "["; w; "]"
FIELD #1, 2 AS c, 2 AS d
PRINT "7: ["; c; "]["; d; "]";: FIELD #1, 4 AS a: PRINT "["; a; "]"; LEN(a);: LSET w = "abcdefgh": PRINT "["; a; "]["; b; "]["; c; "]"
a = "QQ"
PRINT "8: ["; a; "]"; LEN(a); "["; w; "]";: LSET w = "ABCDEFGH": PRINT "["; a; "]"; LEN(a);
a = "longer than the field": PRINT LEN(a);: GET #1, 1: PRINT "["; a; "]"; LEN(a)
b = "": PRINT "9:"; LEN(b);: LSET b = "k": PRINT LEN(b); "["; w; "]"
LSET w = "ABCDEFGH": PUT #1, 1
PRINT "10:";: l = 0: GET #1, 1, l: PRINT l; "["; w; "]";: x = "ijklmnop": PUT #1, 1, x: PRINT "["; w; "]";: GET #1, 1: PRINT "["; w; "]"
FIELD #1, 0 AS c, 8 AS d
PRINT "11:"; LEN(c); "["; d; "]";: LSET c = "x": PRINT LEN(c)
CLOSE #1
PRINT "12: ["; w; "]"; LEN(w);: LSET w = "closed": PRINT "["; w; "]";: w = "free": PRINT "["; w; "]"
OPEN f FOR RANDOM AS #1 LEN = 8
PRINT "13:"; LOF(1); "["; d; "]";: LSET d = "again": PUT #1, 1: FIELD #1, 8 AS w: GET #1, 1: PRINT ASC(w); "["; d; "]"
PRINT "14:";: FIELD #1, 20 AS a: PRINT LEN(a);: FIELD #1, 5 AS a, 4 AS b, 1 AS c: PRINT LEN(a); LEN(b); LEN(c);
FIELD #1, -1 AS c: PRINT LEN(c);: FIELD #1, 2.5 AS c: PRINT LEN(c);: FIELD #1, 3.5 AS c: PRINT LEN(c)
k = 0: s = 2.6
PRINT "15:";: FIELD 1, 4 AS c: PRINT LEN(c);: FIELD #k + 1, s + 2 AS c: PRINT LEN(c);: FIELD (1), 7 AS c: PRINT LEN(c);
FIELD #3, 2 AS c: PRINT LEN(c);: FIELD #0, 2 AS c: PRINT LEN(c);: FIELD #-1, 2 AS c: PRINT LEN(c)
FIELD #1, 2 AS e(1), 3 AS e(2), 3 AS r.v
LSET e(1) = "ab": LSET e(2) = "cde": LSET r.v = "fgh"
FIELD #1, 8 AS a
PRINT "16:"; LEN(e(1)); LEN(e(2)); LEN(e(3)); LEN(r.v); "["; a; "]";: k = 2: FIELD #1, 4 AS e(k + 1), 4 AS q(k).v: PRINT "["; e(3); "]["; q(2).v; "]";
FIELD #1, 2 AS e(4): PRINT
PRINT "17:";: FIELD #fn&, wd& AS c: PRINT LEN(c);: FIELD #fn&, wd& AS e(ix&): PRINT LEN(e(1))
PRINT "18:";: GET #1, 0: PRINT SEEK(1);: PUT #1, -1: PRINT SEEK(1);: GET #3: PUT #3: PRINT
CLOSE #1
OPEN f FOR BINARY AS #1: PRINT "19:";: FIELD #1, 2 AS c: GET #1: PUT #1: CLOSE #1
OPEN "s86_field_lset_2.tmp" FOR OUTPUT AS #1: FIELD #1, 2 AS c: CLOSE #1: KILL "s86_field_lset_2.tmp"
OPEN f FOR RANDOM ACCESS READ AS #1 LEN = 8: FIELD #1, 8 AS c: GET #1, 1: PUT #1, 1: CLOSE #1
OPEN f FOR RANDOM ACCESS WRITE AS #1 LEN = 8: FIELD #1, 8 AS c: GET #1, 1: PUT #1, 1: CLOSE #1
PRINT
OPEN f FOR BINARY AS #1
PRINT "20:";: FOR k = 1 TO LOF(1): GET #1, k, ub: PRINT ub;: NEXT: PRINT
CLOSE #1: KILL f
a = SPACE$(5)
LSET a = "ab": PRINT "21: ["; a; "]";: RSET a = "ab": PRINT "["; a; "]";: LSET a = "abcde": PRINT "["; a; "]";: RSET a = "vwxyz": PRINT "["; a; "]"
a = SPACE$(3)
LSET a = "abcdef": PRINT "22: ["; a; "]";: RSET a = "abcdef": PRINT "["; a; "]";: LSET a = "": PRINT "["; a; "]";
a = "": LSET a = "abc": PRINT "["; a; "]"; LEN(a);: RSET a = "abc": PRINT LEN(a)
LSET y = "ab": PRINT "23: ["; y; "]";: RSET y = "ab": PRINT "["; y; "]";: RSET y = "abcdefgh": PRINT "["; y; "]";: LSET y = "": PRINT "["; y; "]"
e(2) = ".....": k = 1
LSET e(2) = "ab": PRINT "24: ["; e(2); "]";: RSET e(k + 1) = "cd": PRINT "["; e(2); "]";: RSET fx(1) = "z": PRINT "["; fx(1); "]";: LSET e(4) = "ab": PRINT
r.v = "12345": q(2).v = "1234"
LSET r.s = "ab": PRINT "25: ["; r.s; "]";: RSET r.s = "ab": PRINT "["; r.s; "]";: LSET r.v = "ab": PRINT "["; r.v; "]";: RSET q(1).s = "ab": RSET q(2).v = "a": PRINT "["; q(1).s; "]["; q(2).v; "]"
a = "abcdef": LSET a = MID$(a, 3): PRINT "26: ["; a; "]";: a = "abcdef": RSET a = LEFT$(a, 2): PRINT "["; a; "]";: LSET a = a + "x": PRINT "["; a; "]"
e(1) = "111"
PRINT "27:";: LSET e(ix&) = vl$: PRINT " ["; e(1); "]";: a = SPACE$(8): RSET a = r.s: PRINT "["; a; "]";: LSET y = a: PRINT "["; y; "]"
a = "keep."
PRINT "28:";: LSET a = CHR$(-1): PRINT "["; a; "]";: IF LEN(a) THEN LSET a = "x" ELSE RSET a = "y"
PRINT "["; a; "]"
SYSTEM
handler:
PRINT " <error"; ERR; ">";
RESUME NEXT
FUNCTION fn&
    PRINT " fn";
    fn& = 1
END FUNCTION
FUNCTION wd&
    PRINT " wd";
    wd& = 3
END FUNCTION
FUNCTION ix&
    PRINT " ix";
    ix& = 1
END FUNCTION
FUNCTION vl$
    PRINT " vl";
    vl$ = "v"
END FUNCTION
