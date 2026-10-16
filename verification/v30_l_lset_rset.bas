$CONSOLE:ONLY
' Verification (step 4 of M3, first task group of m3-console-files): LSET and RSET on strings that are no field:
' a shorter, an equal and a longer value, an empty target, an empty value, a fixed-length string, an array
' element, a member, a value that is the target itself or names it, the order of target and value, a value of
' another kind of string, an error in the value.
ON ERROR GOTO h
TYPE t
    s AS STRING * 6
    v AS STRING
    n AS LONG
END TYPE
DIM a AS STRING, x AS STRING * 5, e(1 TO 3) AS STRING, r AS t, q(2) AS t, k AS LONG, fx(2) AS STRING * 4
a = SPACE$(5)
LSET a = "ab": PRINT "["; a; "]"; LEN(a)
RSET a = "ab": PRINT "["; a; "]"; LEN(a)
LSET a = "abcde": PRINT "["; a; "]"
RSET a = "vwxyz": PRINT "["; a; "]"
a = SPACE$(3)
LSET a = "abcdef": PRINT "["; a; "]"; LEN(a)
RSET a = "abcdef": PRINT "["; a; "]"; LEN(a)
LSET a = "": PRINT "["; a; "]"; LEN(a)
a = "xyz": RSET a = "": PRINT "["; a; "]"; LEN(a)
a = ""
LSET a = "abc": PRINT "["; a; "]"; LEN(a)
RSET a = "abc": PRINT "["; a; "]"; LEN(a)
PRINT "a fixed-length string"
LSET x = "ab": PRINT "["; x; "]"
RSET x = "ab": PRINT "["; x; "]"
RSET x = "abcdefgh": PRINT "["; x; "]"
LSET x = "": PRINT "["; x; "]"
PRINT "an array element"
e(2) = "....."
LSET e(2) = "ab": PRINT "["; e(2); "]"
RSET e(2) = "ab": PRINT "["; e(2); "]"
k = 1
LSET e(k + 1) = "cd": PRINT "["; e(2); "]"
RSET fx(1) = "z": PRINT "["; fx(1); "]"
PRINT "an element out of range:";: LSET e(4) = "ab": PRINT
PRINT "a member"
LSET r.s = "ab": PRINT "["; r.s; "]"
RSET r.s = "ab": PRINT "["; r.s; "]"
r.v = "12345"
LSET r.v = "ab": PRINT "["; r.v; "]"
RSET r.v = "ab": PRINT "["; r.v; "]"
RSET q(1).s = "ab": PRINT "["; q(1).s; "]"
q(2).v = "1234": RSET q(2).v = "a": PRINT "["; q(2).v; "]"
PRINT "the target in the value"
a = "abcdef"
LSET a = MID$(a, 3): PRINT "["; a; "]"
a = "abcdef"
RSET a = LEFT$(a, 2): PRINT "["; a; "]"
a = "abcdef"
RSET a = a: PRINT "["; a; "]"
LSET a = a + "x": PRINT "["; a; "]"
PRINT "the order of target and value"
e(1) = "111": e(2) = "222"
LSET e(ix&) = vl$: PRINT "["; e(1); "]"
PRINT "a value of another kind"
a = SPACE$(8)
LSET a = x: PRINT "["; a; "]"
RSET a = r.s: PRINT "["; a; "]"
LSET x = a: PRINT "["; x; "]"
PRINT "a comparison as the value's tail"
a = "....."
LSET a = "ab" + "c": PRINT "["; a; "]"
PRINT "an error in the value:";
a = "keep."
LSET a = CHR$(-1): PRINT "["; a; "]"
PRINT "an error in the value:";
RSET a = bad$: PRINT "["; a; "]"
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
FUNCTION ix&
    PRINT "ix ";
    ix& = 1
END FUNCTION
FUNCTION vl$
    PRINT "vl ";
    vl$ = "v"
END FUNCTION
FUNCTION bad$
    ERROR 5
    bad$ = "bad"
END FUNCTION
