$CONSOLE:ONLY
' Slice (m3-redim-retain task 3.2): REDIM _RETAIN of the other kinds of element and in other places
' (verification\v34_b, v34_c): strings in one and two dimensions, STRING * 3 (NUL bytes where nothing is kept),
' _UNSIGNED _BIT * 5 and _BIT, a user type with strings, one with a member array, REDIM _RETAIN SHARED, a local
' array of a SUB, a SHARED array in a SUB, an array parameter, a static array (error 10), a member array (nothing
' changes), conventional memory, and a _MEM block of the array, no longer valid after it.
TYPE rec
    n AS LONG
    s AS STRING
    f AS STRING * 3
END TYPE
TYPE box
    id AS INTEGER
    v(1 TO 3) AS LONG
    t AS STRING * 2
END TYPE
DECLARE SUB grow (p() AS LONG)
ON ERROR GOTO h
DIM i AS LONG, j AS LONG
REDIM _RETAIN SHARED g(1 TO 3) AS LONG
g(2) = 22
PRINT "strings:";
REDIM s(1 TO 4) AS STRING
FOR i = 1 TO 4: s(i) = CHR$(64 + i): NEXT
REDIM _RETAIN s(3 TO 6) AS STRING
FOR i = 3 TO 6: PRINT "["; s(i); "]";: NEXT: PRINT
PRINT "strings, two dimensions:"
REDIM t(2, 2) AS STRING
FOR i = 0 TO 2: FOR j = 0 TO 2: t(i, j) = LTRIM$(STR$(i)) + LTRIM$(STR$(j)): NEXT: NEXT
REDIM _RETAIN t(1 TO 3, 1 TO 2) AS STRING
FOR i = 1 TO 3: FOR j = 1 TO 2: PRINT "["; t(i, j); "]";: NEXT: PRINT: NEXT
PRINT "STRING * 3:";
REDIM x(1 TO 3) AS STRING * 3
FOR i = 1 TO 3: x(i) = STRING$(3, 96 + i): NEXT
REDIM _RETAIN x(2 TO 5) AS STRING * 3
FOR i = 2 TO 5: PRINT ASC(x(i), 1); ASC(x(i), 3);: NEXT: PRINT
PRINT "_UNSIGNED _BIT * 5:";
REDIM p(0 TO 9) AS _UNSIGNED _BIT * 5
FOR i = 0 TO 9: p(i) = 3 * i: NEXT
REDIM _RETAIN p(5 TO 14) AS _UNSIGNED _BIT * 5
FOR i = 5 TO 14: PRINT p(i);: NEXT: PRINT
PRINT "_BIT:"
REDIM q(3, 3) AS _BIT
FOR i = 0 TO 3: FOR j = 0 TO 3: q(i, j) = -((i + j) MOD 2): NEXT: NEXT
REDIM _RETAIN q(1 TO 4, 2 TO 5) AS _BIT
FOR i = 1 TO 4: FOR j = 2 TO 5: PRINT q(i, j);: NEXT: PRINT: NEXT
PRINT "a user type with strings:";
REDIM r(1 TO 3) AS rec
FOR i = 1 TO 3: r(i).n = i: r(i).s = "s" + LTRIM$(STR$(i)): r(i).f = "f" + LTRIM$(STR$(i)): NEXT
REDIM _RETAIN r(2 TO 4) AS rec
FOR i = 2 TO 4: PRINT r(i).n; "["; r(i).s; "]"; ASC(r(i).f, 1); ASC(r(i).f, 3);: NEXT: PRINT
PRINT "a user type with a member array:"
REDIM b(1, 1) AS box
FOR i = 0 TO 1: FOR j = 0 TO 1
        b(i, j).id = 10 * i + j: b(i, j).v(2) = 100 + 10 * i + j: b(i, j).t = "t" + LTRIM$(STR$(j))
NEXT: NEXT
REDIM _RETAIN b(0 TO 1, 1 TO 2) AS box
FOR i = 0 TO 1: FOR j = 1 TO 2: PRINT b(i, j).id; b(i, j).v(2); ASC(b(i, j).t, 1);: NEXT: PRINT: NEXT
PRINT "a member array:";
DIM one AS box
one.v(2) = 5
REDIM _RETAIN one.v(1 TO 3) AS LONG
PRINT one.v(2)
PRINT "a local array:";
localsub
PRINT "SHARED:";
gsub
PRINT UBOUND(g); g(2);
REDIM k(1 TO 3) AS LONG
k(2) = 33
sharedsub
PRINT UBOUND(k); k(2)
PRINT "a parameter:";
REDIM w(1 TO 3) AS LONG
FOR i = 1 TO 3: w(i) = i: NEXT
grow w()
FOR i = 0 TO 5: PRINT w(i);: NEXT: PRINT
PRINT "a static array:";
DIM st(5) AS LONG
st(1) = 3
REDIM _RETAIN st(9) AS LONG
PRINT UBOUND(st); st(1)
PRINT "conventional memory:";
REDIM cm(1 TO 4) AS INTEGER
FOR i = 1 TO 4: cm(i) = i * 11: NEXT
PRINT VARPTR(cm(1)) >= 0;
REDIM _RETAIN cm(2 TO 6) AS INTEGER
FOR i = 2 TO 6: PRINT cm(i);: NEXT: PRINT
PRINT "a _MEM block:";
REDIM mm(1 TO 3) AS LONG
DIM blk AS _MEM
blk = _MEM(mm())
PRINT _MEMEXISTS(blk);
REDIM _RETAIN mm(1 TO 5) AS LONG
PRINT _MEMEXISTS(blk)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB localsub
    REDIM l(1 TO 3) AS STRING
    l(3) = "x"
    REDIM _RETAIN l(3 TO 4) AS STRING
    PRINT "["; l(3); "]"; LBOUND(l)
END SUB

SUB gsub
    REDIM _RETAIN g(2 TO 8) AS LONG
END SUB

SUB sharedsub
    SHARED k() AS LONG
    REDIM _RETAIN k(2 TO 7) AS LONG
END SUB

SUB grow (p() AS LONG)
    REDIM _RETAIN p(0 TO 5) AS LONG
END SUB
