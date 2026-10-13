$CONSOLE:ONLY
' Slice (m3-console-files, task 4.3; from verification\v30_c_using_file): PRINT USING to a file, read back line by
' line, and LPRINT as code that is compiled, linked and not reached (it would print a page; what it compiles to is
' compared by tests\callsite). The file's lines: values, the format starting again, a comma between values, a
' trailing ; and a trailing , (no line end), strings, items before USING (with a zone, which a file takes), TAB and
' SPC among the values, a value of the wrong kind and a value that is not finite (what was built is written with
' no line end). Then the errors of the file: open for input (54), not open (52), an error in a value before the
' file's. The file is s82_print_using_file.tmp and is removed.
ON ERROR GOTO handler
DIM a AS STRING, f AS STRING, n AS LONG, d AS DOUBLE
f = "s82_print_using_file.tmp"
OPEN f FOR OUTPUT AS #1
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
n = 1
PRINT #n, USING "<&>"; f
PRINT #1, USING "###"; "bad"
PRINT #1, "after"
d = 0: d = 1 / d
PRINT #1, USING "## ## ##"; 1; d; 3
PRINT #1, "after"
CLOSE #1
OPEN f FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, a
    PRINT "["; a; "]"
LOOP
PRINT "open for input:";: PRINT #1, USING "##"; 1: PRINT
CLOSE #1
PRINT "not open:";: PRINT #1, USING "##"; 1: PRINT
PRINT "an error in a value first:";: PRINT #2, USING "##"; ASC(""): PRINT
KILL f
PRINT "done"
SYSTEM

' Not reached: the printer.
LPRINT
LPRINT "a"; n, d
LPRINT TAB(5); "x"; SPC(2); "y";
LPRINT USING "##.##"; d
LPRINT "total"; USING " ### &"; n; f;
IF n THEN LPRINT "in an IF" ELSE LPRINT USING "&"; f
handler:
PRINT "<error"; ERR; ">";
RESUME NEXT
