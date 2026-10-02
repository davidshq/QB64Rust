$CONSOLE:ONLY
' v10: PRINT / PRINT USING / WRITE / PRINT # / WRITE # / INPUT # emission (study\10 section 2)
' Lines are bracketed with [ ] so leading and trailing spaces are visible.
ON ERROR GOTO eh

PRINT "1. numbers:"
PRINT "["; 5; -3; 1.5; "]"
PRINT "2. auto-semicolon:"
x = 7
PRINT "[" "a"1; x "b" "]"
PRINT "3. USING in the middle of PRINT:"
PRINT "["; USING "##.#"; 3.14159; : PRINT "]"
PRINT "["; x; USING "###"; 42; : PRINT "]"
' 4. (removed) PRINT with a comma on a redirected $CONSOLE never terminates: tab() loops on POS, which reads the
'    console window cursor, not stdout. See study\10 section 2.
PRINT "5. WRITE to screen:"
WRITE 1, -2.5, "q" + CHR$(34) + "x", "s"
PRINT "6. PRINT USING after an earlier PRINT, in a SUB (tqbs reuse):"
usingsub
PRINT "7. file output:"
OPEN "v10_tmp.txt" FOR OUTPUT AS #1
PRINT #1, 1; -2; "a"
PRINT #1, "x", "y"; 3, 4
WRITE #1, 1, -2.5, "q" + CHR$(34) + "x"
WRITE #1, 1, 2,
WRITE #1, "after"
PRINT #1, USING "##.##"; 3.14159; 2.5
PRINT #1, "b"; USING "#"; 7
PRINT #1,
CLOSE #1
OPEN "v10_tmp.txt" FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, l$
    PRINT "["; l$; "]"
LOOP
CLOSE #1
PRINT "8. INPUT # items:"
OPEN "v10_tmp.txt" FOR OUTPUT AS #1
PRINT #1, "  abc  , 12 ," + CHR$(34) + "q,x" + CHR$(34) + " , 3e2 , &H10"
CLOSE #1
OPEN "v10_tmp.txt" FOR INPUT AS #1
INPUT #1, a$, n, b$, d#, h
PRINT "["; a$; "]["; n; "]["; b$; "]["; d#; "]["; h; "]"
CLOSE #1
KILL "v10_tmp.txt"
SYSTEM

eh:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB usingsub
    DIM t AS STRING, u AS STRING
    t = "keep-t": u = "keep-u"
    PRINT USING "[##]"; 5
    PRINT t; " "; u
END SUB
