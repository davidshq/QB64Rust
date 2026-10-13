$CONSOLE:ONLY
' Slice (m3-get-put-bload, task 3.1): TAB and SPC as items of PRINT and PRINT #, and as the functions they are
' (verification\v28_a_tab_spc). On the console POS(0) is always 1, so TAB(n) is n - 1 blanks wherever the line
' stands; in a file the column is the file's. No comma follows an item of a console PRINT: with redirected output
' that never ends in the old compiler's program (study\00 §5). Then _RGB32, _RGBA32, DATE$ and TIME$ by what
' does not change from run to run, and _FULLSCREEN as code that is compiled and not reached.
DIM i AS INTEGER, s AS SINGLE, t AS STRING, k AS STRING, u AS _UNSIGNED LONG, q AS _INTEGER64
PRINT "a"; TAB(5); "b"; TAB(10); "c"
PRINT "abcdefgh"; TAB(5); "below"
PRINT "a"; SPC(3); "b"; SPC(0); "c"
PRINT TAB(3); 7; TAB(12); -7
PRINT TAB(5)
PRINT TAB(5); : PRINT "x"
PRINT TAB(5) "string after"
PRINT "x" TAB(8) "y" SPC(2) "z"
i = 4: s = 6.5: q = 3
PRINT TAB(i); "i"; TAB(s); "s"; TAB(2.5); "r"; SPC(q); "q"
PRINT TAB(0); "zero"; TAB(-3); "negative"
PRINT "123456789"; TAB(10); "at ten"; TAB(10); "again"
PRINT "wide"; TAB(85); "past the width"
PRINT SPC(85); "spc past the width"
PRINT "p"; SPC(-2); "q"
PRINT POS(0); TAB(20); POS(0); SPC(5); POS(0)
' As functions.
t = TAB(6)
PRINT "["; t; "]"; LEN(t)
t = SPC(4)
PRINT "["; t; "]"; LEN(t)
PRINT LEN(SPC(3) + TAB(9)); LEN(TAB(2) + TAB(2)); LEN(SPC(i) + SPC(s))
PRINT "abcdef"; : t = TAB(3): PRINT LEN(t); ASC(t)
WRITE TAB(3), SPC(2)
' To a file.
OPEN "s71_tab_spc.tmp" FOR OUTPUT AS #1
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
OPEN "s71_tab_spc.tmp" FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, k
    PRINT "["; k; "]"
LOOP
CLOSE #1
KILL "s71_tab_spc.tmp"
' Colours of 32 bits: red, green and blue with full alpha; one value as a grey; with an alpha.
u = _RGB32(1, 2, 3)
PRINT HEX$(u); " "; HEX$(_RGB32(255, 128, 0)); " "; HEX$(_RGB32(5)); " "; HEX$(_RGB32(5, 16)); " "; HEX$(_RGB32(1, 2, 3, 4))
PRINT HEX$(_RGBA32(1, 2, 3, 4)); " "; HEX$(_RGBA32(300, -1, s, i)); _RGB32(0, 0, 0); _RGB32(255, 255, 255) + 1
i = 200: s = 100.5
PRINT HEX$(_RGB32(i, s, q)); " "; HEX$(_RGB32(i * 2, s, q))
' The date and the time have their length and their separators.
t = DATE$: k = TIME$
PRINT LEN(t); MID$(t, 3, 1); MID$(t, 6, 1); LEN(k); MID$(k, 3, 1); MID$(k, 6, 1); LEN(DATE$ + TIME$)
SYSTEM
_FULLSCREEN
_FULLSCREEN _STRETCH, _SMOOTH
_FULLSCREEN _OFF
i = _FULLSCREEN
