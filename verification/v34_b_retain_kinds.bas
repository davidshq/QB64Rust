$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): REDIM _RETAIN of the other kinds of element: variable
' strings, fixed-length strings, _BIT and _UNSIGNED _BIT * n, a user type with a string, a user type with a
' member array and a STRING * n member.
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
ON ERROR GOTO h
DIM i AS LONG, j AS LONG
PRINT "1. strings s(1 TO 4) = A..D, then s(3 TO 6):";
REDIM s(1 TO 4) AS STRING
FOR i = 1 TO 4: s(i) = CHR$(64 + i): NEXT
REDIM _RETAIN s(3 TO 6) AS STRING
FOR i = 3 TO 6: PRINT "["; s(i); "]";: NEXT: PRINT
PRINT "2. strings in two dimensions, t(2, 2) = ij, then t(1 TO 3, 1 TO 2):"
REDIM t(2, 2) AS STRING
FOR i = 0 TO 2: FOR j = 0 TO 2: t(i, j) = LTRIM$(STR$(i)) + LTRIM$(STR$(j)): NEXT: NEXT
REDIM _RETAIN t(1 TO 3, 1 TO 2) AS STRING
FOR i = 1 TO 3: FOR j = 1 TO 2: PRINT "["; t(i, j); "]";: NEXT: PRINT: NEXT
PRINT "3. STRING * 3, x(1 TO 3) = aaa, bbb, ccc, then x(2 TO 5):";
REDIM x(1 TO 3) AS STRING * 3
FOR i = 1 TO 3: x(i) = STRING$(3, 96 + i): NEXT
REDIM _RETAIN x(2 TO 5) AS STRING * 3
FOR i = 2 TO 5: PRINT "["; x(i); "]"; ASC(x(i));: NEXT: PRINT
PRINT "4. _UNSIGNED _BIT * 5, p(0 TO 9) = 3*i, then p(5 TO 14):";
REDIM p(0 TO 9) AS _UNSIGNED _BIT * 5
FOR i = 0 TO 9: p(i) = 3 * i: NEXT
REDIM _RETAIN p(5 TO 14) AS _UNSIGNED _BIT * 5
FOR i = 5 TO 14: PRINT p(i);: NEXT: PRINT
PRINT "5. _BIT, two dimensions, q(3, 3) = -((i+j) MOD 2), then q(1 TO 4, 2 TO 5):"
REDIM q(3, 3) AS _BIT
FOR i = 0 TO 3: FOR j = 0 TO 3: q(i, j) = -((i + j) MOD 2): NEXT: NEXT
REDIM _RETAIN q(1 TO 4, 2 TO 5) AS _BIT
FOR i = 1 TO 4: FOR j = 2 TO 5: PRINT q(i, j);: NEXT: PRINT: NEXT
PRINT "6. a user type with strings, r(1 TO 3), then r(2 TO 4):"
REDIM r(1 TO 3) AS rec
FOR i = 1 TO 3: r(i).n = i: r(i).s = "s" + LTRIM$(STR$(i)): r(i).f = "f" + LTRIM$(STR$(i)): NEXT
REDIM _RETAIN r(2 TO 4) AS rec
FOR i = 2 TO 4: PRINT r(i).n; "["; r(i).s; "]["; r(i).f; "]";: NEXT: PRINT
PRINT "7. a user type with a member array, b(1, 1), then b(0 TO 1, 1 TO 2):"
REDIM b(1, 1) AS box
FOR i = 0 TO 1: FOR j = 0 TO 1
        b(i, j).id = 10 * i + j: b(i, j).v(2) = 100 + 10 * i + j: b(i, j).t = "t" + LTRIM$(STR$(j))
NEXT: NEXT
REDIM _RETAIN b(0 TO 1, 1 TO 2) AS box
FOR i = 0 TO 1: FOR j = 1 TO 2: PRINT b(i, j).id; b(i, j).v(2); "["; b(i, j).t; "]";: NEXT: PRINT: NEXT
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
