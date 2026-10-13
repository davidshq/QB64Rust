' TEST: cpp
$CONSOLE:ONLY
' The C++ of PRINT USING (m3-console-files task 4.1, design D1; read with qb64pe -z, study\00 section 5): the
' statement's string and its place in the format, the format copied once, one call per value by its type, the
' jump to the second label after each, and the block at that label that prints what was built. Line 9 prints
' two values; line 10 goes to a file (it leaves by its first label after the format); line 11 is LPRINT
DIM d AS DOUBLE, v AS STRING
OPEN "print_using_cpp.tmp" FOR OUTPUT AS #1
PRINT USING "##.## &"; d; v
PRINT #1, "x"; USING v; 5; TAB(3); d;
IF d THEN LPRINT "a"; USING "##"; d
CLOSE #1
SYSTEM
