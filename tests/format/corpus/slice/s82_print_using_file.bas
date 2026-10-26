$Console:Only
' Slice (m3-console-files, task 4.3; from verification\v30_c_using_file): PRINT USING to a file, read back line by
' line, and LPRINT as code that is compiled, linked and not reached (it would print a page; what it compiles to is
' compared by tests\callsite). The file's lines: values, the format starting again, a comma between values, a
' trailing ; and a trailing , (no line end), strings, items before USING (with a zone, which a file takes), TAB and
' SPC among the values, a value of the wrong kind and a value that is not finite (what was built is written with
' no line end). Then the errors of the file: open for input (54), not open (52), an error in a value before the
' file's. The file is s82_print_using_file.tmp and is removed.
On Error GoTo handler
Dim a As String, f As String, n As Long, d As Double
f = "s82_print_using_file.tmp"
Open f For Output As #1
Print #1, Using "##.##"; 3.14159
Print #1, Using "## ##"; 1; 2
Print #1, Using "##"; 1; 2; 3
Print #1, Using "##"; 1, 2, 3
Print #1, Using "##"; 1;
Print #1, "|"
Print #1, Using "##"; 1,
Print #1, "|"
Print #1, Using "& and &"; "a"; "b"
Print #1, "x"; Using "##"; 5
Print #1, "x", Using "##"; 5
Print #1, 1; Using "##"; 5; 6
Print #1, Using "## ##"; 1; Tab(10); 2
Print #1, Using "## ##"; 1; Spc(3); 2
n = 1
Print #n, Using "<&>"; f
Print #1, Using "###"; "bad"
Print #1, "after"
d = 0: d = 1 / d
Print #1, Using "## ## ##"; 1; d; 3
Print #1, "after"
Close #1
Open f For Input As #1
Do Until EOF(1)
    Line Input #1, a
    Print "["; a; "]"
Loop
Print "open for input:";: Print #1, Using "##"; 1: Print
Close #1
Print "not open:";: Print #1, Using "##"; 1: Print
Print "an error in a value first:";: Print #2, Using "##"; Asc(""): Print
Kill f
Print "done"
System

' Not reached: the printer.
LPrint
LPrint "a"; n, d
LPrint Tab(5); "x"; Spc(2); "y";
LPrint Using "##.##"; d
LPrint "total"; Using " ### &"; n; f;
If n Then LPrint "in an IF" Else LPrint Using "&"; f
handler:
Print "<error"; Err; ">";
Resume Next
