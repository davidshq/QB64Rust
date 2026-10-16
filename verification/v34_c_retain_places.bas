$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): REDIM _RETAIN in other places: a local array of a SUB,
' REDIM _RETAIN SHARED, an array parameter, a SHARED array in a SUB, a static array (error 10), a bound that
' raises, an array in conventional memory (VARPTR), a _MEM block of the array.
DECLARE SUB grow (p() AS LONG)
ON ERROR GOTO h
DIM i AS LONG
REDIM _RETAIN SHARED g(1 TO 3) AS LONG
g(2) = 22
PRINT "1. a local array of a SUB:";
localsub
PRINT "2. REDIM _RETAIN SHARED, then in a SUB; a SHARED array in a SUB:";
gsub
PRINT UBOUND(g); g(2);
REDIM k(1 TO 3) AS LONG
k(2) = 33
sharedsub
PRINT UBOUND(k); k(2)
PRINT "3. an array parameter, p(1 TO 3) = 1..3 grown to p(0 TO 5):";
REDIM w(1 TO 3) AS LONG
FOR i = 1 TO 3: w(i) = i: NEXT
grow w()
FOR i = 0 TO 5: PRINT w(i);: NEXT: PRINT
PRINT "4. a static array:";
DIM st(5) AS LONG
st(1) = 3
REDIM _RETAIN st(9) AS LONG
PRINT UBOUND(st); st(1)
PRINT "5. a bound that raises:";
REDIM e(1 TO 3) AS LONG: e(2) = 5
REDIM _RETAIN e(1 TO ASC("")) AS LONG
PRINT " then:"; LBOUND(e); UBOUND(e); e(2)
PRINT "6. conventional memory, VARPTR:";
REDIM cm(1 TO 4) AS INTEGER
FOR i = 1 TO 4: cm(i) = i * 11: NEXT
PRINT VARPTR(cm(1)) >= 0;
REDIM _RETAIN cm(2 TO 6) AS INTEGER
FOR i = 2 TO 6: PRINT cm(i);: NEXT: PRINT
PRINT "7. a _MEM block of the array:";
REDIM mm(1 TO 3) AS LONG
DIM blk AS _MEM
blk = _MEM(mm())
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

