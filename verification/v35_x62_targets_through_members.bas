$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 4.2): READ, INPUT #, LINE INPUT #, console INPUT and the MID$ statement into
' a member of an array element and into an element of a member array (fixed and _DYNAMIC): whether the index is
' evaluated before or after the item is read (the index FUNCTION reads an item itself), and where the item goes
' when an index is outside its bounds.
TYPE t
    m AS LONG
    s AS STRING
    v(1) AS LONG
    w(1) AS STRING
    d(1) _DYNAMIC AS LONG
    e(1) _DYNAMIC AS STRING
END TYPE
DIM SHARED a(2) AS t
DIM SHARED q$, z$
ON ERROR GOTO h
READ a(rd(1)).m: PRINT " 1."; a(1).m
READ a(rd(1)).s: PRINT " 2. "; a(1).s
READ a(1).v(rd(1)): PRINT " 3."; a(1).v(1)
READ a(1).w(rd(1)): PRINT " 4. "; a(1).w(1)
READ a(1).d(rd(1)): PRINT " 5."; a(1).d(1)
READ a(1).e(rd(1)): PRINT " 6. "; a(1).e(1)
READ a(9).m: PRINT " 7."; a(0).m
READ a(9).s: PRINT " 8. "; a(0).s
READ a(1).v(9): PRINT " 9."; a(1).v(0)
READ a(1).d(9): PRINT "10."; a(1).d(0)
READ a(1).e(9): PRINT "11. "; a(1).e(0)
READ q$: PRINT "12. next item: "; q$
OPEN "v35_x62.tmp" FOR OUTPUT AS #1
FOR i = 1 TO 20: PRINT #1, "L"; LTRIM$(STR$(100 + i)): NEXT
CLOSE #1
OPEN "v35_x62.tmp" FOR INPUT AS #1
INPUT #1, a(rf(1)).s: PRINT "13. "; a(1).s
INPUT #1, a(1).e(rf(1)): PRINT "14. "; a(1).e(1)
LINE INPUT #1, a(rf(1)).s: PRINT "15. "; a(1).s
LINE INPUT #1, a(1).w(rf(1)): PRINT "16. "; a(1).w(1)
INPUT #1, a(9).s: PRINT "17. "; a(0).s
LINE INPUT #1, a(1).e(9): PRINT "18. "; a(1).e(0)
INPUT #1, q$: PRINT "19. next line: "; q$
CLOSE #1
OPEN "v35_x62.tmp" FOR OUTPUT AS #1
FOR i = 1 TO 20: PRINT #1, 200 + i: NEXT
CLOSE #1
OPEN "v35_x62.tmp" FOR INPUT AS #1
INPUT #1, a(rf(1)).m: PRINT "20."; a(1).m
INPUT #1, a(1).d(rf(1)): PRINT "21."; a(1).d(1)
INPUT #1, a(9).m: PRINT "22."; a(0).m
INPUT #1, a(1).v(9): PRINT "23."; a(1).v(0)
INPUT #1, q$: PRINT "24. next line: "; q$
CLOSE #1
KILL "v35_x62.tmp"
INPUT a(ci(1)).m, a(1).d(ci(1)): PRINT "25."; a(1).m; a(1).d(1)
LINE INPUT a(1).e(ci(1)): PRINT "26. "; a(1).e(1)
INPUT a(9).m: PRINT "27."; a(0).m
LINE INPUT a(1).e(9): PRINT "28. "; a(1).e(0)
a(1).s = "abcdef": a(1).e(1) = "uvwxyz": a(0).s = "000000": a(1).w(0) = "nnnnnn"
z$ = "A": MID$(a(mf(1)).s, 1, 1) = z$: PRINT "29. "; a(1).s
z$ = "A": MID$(a(1).e(mf(1)), 2) = z$: PRINT "30. "; a(1).e(1)
MID$(a(9).s, 3, 1) = "Z": PRINT "31. "; a(0).s
MID$(a(1).w(9), 4, 1) = "Z": PRINT "32. "; a(1).w(0)
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
DATA 10,11,x,y,20,21,p,q,30,31,r,s,40,u,50,v,60,w,70,z,80,y,90
FUNCTION rd (k AS LONG)
    READ q$
    PRINT "[rd "; q$; "]";
    rd = k
END FUNCTION
FUNCTION rf (k AS LONG)
    INPUT #1, q$
    PRINT "[rf "; q$; "]";
    rf = k
END FUNCTION
FUNCTION ci (k AS LONG)
    PRINT "[ci]";
    ci = k
END FUNCTION
FUNCTION mf (k AS LONG)
    z$ = "B"
    PRINT "[mf]";
    mf = k
END FUNCTION
