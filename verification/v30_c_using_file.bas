$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): PRINT #n, USING. What is written to the file, with ; and
' , between the values and at the end, USING after other items, a file that is not open, a file open for input,
' and the same to the console with _DEST _CONSOLE and with _ECHO beside it. The file is v30_c.tmp and is removed.
ON ERROR GOTO h
DIM a AS STRING
OPEN "v30_c.tmp" FOR OUTPUT AS #1
PRINT #1, USING "##.##"; 3.14159
PRINT #1, USING "## ##"; 1; 2
PRINT #1, USING "##"; 1; 2; 3
PRINT #1, USING "##"; 1, 2, 3
PRINT #1, USING "##"; 1;
PRINT #1, "|"
PRINT #1, USING "##"; 1,
PRINT #1, "|"
PRINT #1, USING "& and &"; "a"; "b"
PRINT #1, "x"; USING "##"; 5
PRINT #1, "x", USING "##"; 5
PRINT #1, 1; USING "##"; 5; 6
PRINT #1, USING "## ##"; 1; TAB(10); 2
PRINT #1, USING "## ##"; 1; SPC(3); 2
PRINT #1, USING "###"; "bad"
PRINT #1, "after"
PRINT #1, USING "## ## ##"; 1; 1 / 0; 3
PRINT #1, "after"
CLOSE #1
OPEN "v30_c.tmp" FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, a
    PRINT "["; a; "]"
LOOP
PRINT "open for input"
PRINT #1, USING "##"; 1
CLOSE #1
PRINT "not open"
PRINT #1, USING "##"; 1
PRINT #2, USING "##"; 1 / 0
KILL "v30_c.tmp"
PRINT "the console"
_DEST _CONSOLE
PRINT USING "##.##"; 3.14159
_ECHO "echo"
PRINT USING "##"; 1;
PRINT "|"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
