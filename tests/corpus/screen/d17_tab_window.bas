' TAB, SPC and POS in a window: TAB moves to a column, on the next line when the cursor is past it; SPC prints
' blanks; a zone after TAB; a column past the width; TAB and SPC as functions.
DIM t AS STRING
PRINT "a"; TAB(5); "b"; TAB(10); "c"
PRINT "abcdefgh"; TAB(5); "below"
PRINT "a"; SPC(3); "b"; SPC(0); "c"
PRINT TAB(3); 7; TAB(12); -7
PRINT TAB(5), "zone"
PRINT "x" TAB(8) "y" SPC(2) "z"
PRINT POS(0); TAB(20); POS(0); SPC(5); POS(0)
PRINT "wide"; TAB(85); "past the width"
PRINT TAB(0); "zero"; TAB(-3); "negative"
PRINT "p"; SPC(-2); "q"; SPC(90); "after ninety"
PRINT SPC(78); "wrap"
t = TAB(6)
PRINT "["; t; "]"; LEN(t)
PRINT "abcdef"; : t = TAB(3): PRINT LEN(t); ASC(t)
PRINT "abcdef"; : t = SPC(3): PRINT LEN(t); ASC(t)
COLOR 14
PRINT TAB(30); "in yellow"; TAB(60); CSRLIN
SYSTEM
