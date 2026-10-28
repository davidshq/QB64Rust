$CONSOLE:ONLY
' Slice (found by --suite addprefix, QB64pe's $NOPREFIX converter): PUT and GET of a STRING member, of a variable,
' of a shared variable in a SUB and of an array element, to a BINARY file; the member's string went to the C++
' without the parentheses its -> needs, and the build failed.
TYPE tok
    t AS LONG
    c AS STRING
    s AS STRING
END TYPE
DIM SHARED g AS tok
DIM v AS tok, a(1 TO 2) AS tok
v.c = "abc": g.s = "  ": g.c = "de": a(2).c = "fgh"
f$ = "s161.tmp"
OPEN f$ FOR OUTPUT AS #1: CLOSE #1
OPEN f$ FOR BINARY AS #1
PUT #1, , v.c
put_out
PUT #1, , a(2).c
PRINT LOF(1)
v.c = SPACE$(4)
GET #1, 1, v.c
a(1).c = SPACE$(5)
GET #1, , a(1).c
PRINT "["; v.c; "]["; a(1).c; "]"
CLOSE #1
KILL f$
SYSTEM

SUB put_out
    PUT #1, , g.s
    PUT #1, , g.c
END SUB
