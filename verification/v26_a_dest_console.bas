$CONSOLE
$SCREENHIDE
' Verification (step 11): PRINT and WRITE after _DEST _CONSOLE in a program with a hidden window.
_DEST _CONSOLE
PRINT "a"; 1; -2; "b"
' No PRINT with a comma: with standard output redirected, PRINT "z1", "z2" printed "z1" and then blanks without
' end (1.8 MB until the timeout, 2026-10-12; study\00, "Test hygiene"). Under $CONSOLE:ONLY a comma gives zones
' of 10 columns.
PRINT TAB(5); "t"; SPC(3); "s"
PRINT USING "##.##"; 3.14159
WRITE "w", 1
PRINT "no line end";
PRINT
PRINT 1.5; 2 ^ 40; "x" + "y"
SYSTEM
