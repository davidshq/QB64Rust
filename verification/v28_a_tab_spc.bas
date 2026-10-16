$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): TAB and SPC as items of PRINT and PRINT #, and as functions.
DIM i AS INTEGER, s AS SINGLE, t AS STRING, k AS STRING
PRINT "a"; TAB(5); "b"; TAB(10); "c"
PRINT "abcdefgh"; TAB(5); "below"
PRINT "a"; SPC(3); "b"; SPC(0); "c"
PRINT TAB(3); 7; TAB(12); -7
PRINT TAB(5)
PRINT TAB(5); : PRINT "x"
PRINT TAB(5) "string after"
PRINT "x" TAB(8) "y" SPC(2) "z"
i = 4: s = 6.5
PRINT TAB(i); "i"; TAB(s); "s"; TAB(2.5); "r"
PRINT TAB(0); "zero"; TAB(-3); "negative"
PRINT "123456789"; TAB(10); "at ten"; TAB(10); "again"
PRINT "wide"; TAB(85); "past the width"
PRINT "big"; TAB(200); "far"
PRINT SPC(85); "spc past the width"
PRINT "p"; SPC(-2); "q"
PRINT POS(0); TAB(20); POS(0); SPC(5); POS(0)
' As functions.
t = TAB(6)
PRINT "["; t; "]"; LEN(t)
t = SPC(4)
PRINT "["; t; "]"; LEN(t)
PRINT "ab";
t = TAB(6)
PRINT "["; t; "]"; LEN(t)
PRINT LEN(SPC(3) + TAB(9)); LEN(TAB(2) + TAB(2))
PRINT "abcdef"; : t = TAB(3): PRINT LEN(t); ASC(t)
WRITE TAB(3), SPC(2)
' To a file.
OPEN "v28_a_tab_spc.tmp" FOR OUTPUT AS #1
PRINT #1, "a"; TAB(5); "b"; TAB(10); "c"
PRINT #1, "abcdefgh"; TAB(5); "below"
PRINT #1, "a"; SPC(3); "b"
PRINT #1, TAB(5)
PRINT #1, TAB(5);
PRINT #1, "x"
PRINT #1, 7; TAB(12); -7, TAB(30); "z"
PRINT "screen"; : PRINT #1, "f"; TAB(4); "g"
PRINT
CLOSE #1
OPEN "v28_a_tab_spc.tmp" FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, k
    PRINT "["; k; "]"
LOOP
CLOSE #1
KILL "v28_a_tab_spc.tmp"
SYSTEM
