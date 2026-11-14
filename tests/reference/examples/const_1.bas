$CONSOLE:ONLY
CONST MAXARMIES = 12, TITLE$ = "Campaign"
CONST HALF = MAXARMIES / 2
PRINT TITLE$; " allows"; MAXARMIES; "armies, half of them is"; HALF
CONST BIG = 3
PRINT "BIG * 1000000000 ="; BIG * 1000000000
PRINT "2 ^ 3 ^ 2 in a CONST and in a statement:";
CONST P = 2 ^ 3 ^ 2
PRINT P; 2 ^ 3 ^ 2
Report
END

SUB Report
    PRINT "The SUB sees"; MAXARMIES
END SUB
