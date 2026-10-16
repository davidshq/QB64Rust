' TEST: check-fail
$CONSOLE:ONLY
' The forms of PRINT USING and LPRINT the old compiler rejects (m3-console-files tasks 4.1 and 4.2;
' verification\v30_x01-x12, x15-x17, and the forms measured with task 4.1), one line each: each is an error that
' is not "not supported yet"
TYPE t
    n AS LONG
END TYPE
DIM r AS t, v AS STRING, n AS LONG
PRINT USING "##", 5
PRINT USING "##"
PRINT USING "##";
PRINT USING 5; 5
PRINT USING ; 5
PRINT USING "##" 5
PRINT #1, USING "##", 5
PRINT USING "##"; 5 6
PRINT USING "##"; ; 5
PRINT USING "## ##"; 5; ; 6
PRINT USING "##"; 5 USING "#"; 6
PRINT "a"; USING "##"; 5; USING "###"; 6
PRINT USING "##"; r
PRINT #1, USING "##"
PRINT USING #1, "##"; 5
PRINT USING "&&"; "a" "b"
PRINT USING "##"; 5 "a"
PRINT USING "##"; 5;;
PRINT USING "##"; n, , 5
PRINT USING v, 5
PRINT USING n; 5
LPRINT USING "##", 5
LPRINT USING "##"
LPRINT #1, "a"
LPRINT USING "##"; 5 "a"
