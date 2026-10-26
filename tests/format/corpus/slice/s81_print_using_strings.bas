$Console:Only
' Slice (m3-console-files, task 4.3; from verification\v30_b_using_strings): PRINT USING with strings and with
' several values, 17 numbered lines. The string fields, a value of the wrong kind (error 13, what was built is
' printed with no line end), several values and the format starting again, text after the last value used, a
' format with no field (error 5), the end of the statement (a trailing ; or , of either kind), the format read
' once, TAB and SPC among the values, items before USING, an error in a value (the values after it are skipped),
' and a question mark for PRINT. No line prints a zone: a PRINT comma never ends on a redirected console.
On Error GoTo handler
Dim f As String, x As String * 5, n As Long, a(2) As Long
Print "1: [";: Print Using "!"; "hello";: Print "] [";: Print Using "!"; "";: Print "] [";: Print Using "&"; "hello";
Print "] [";: Print Using "&"; "";: Print "]"
Print "2: [";: Print Using "\\"; "hello";: Print "] [";: Print Using "\ \"; "hello";: Print "] [";: Print Using "\  \"; "hello";
Print "] [";: Print Using "\      \"; "hello";: Print "] [";: Print Using "\ab\"; "hello";: Print "]"
x = "ab"
Print "3: [";: Print Using "a&b"; "hello";: Print "] [";: Print Using "_!&"; "hello";: Print "] [";: Print Using "&"; x;
Print "] [";: Print Using "\      \"; x;: Print "] [";: Print Using "& &"; x + "!"; Mid$("abcdef", 2, 3);: Print "]"
Print "4: [";: Print Using "###"; "hello";: Print "] [";: Print Using "&"; 5;: Print "] [";: Print Using "!"; 5;: Print "]"
Print "5: [";: Print Using "### &"; 5; 6;: Print "] [";: Print Using "& ###"; "a"; "b";: Print "] [";
Print Using "ab ### cd &"; 5; 6;: Print "]"
Print "6: [";: Print Using "## ## ##"; 1; 2; 3;: Print "] [";: Print Using "## ## ##"; 1; 2;: Print "] [";
Print Using "## ## ##"; 1;: Print "] [";: Print Using "##"; 1; 2; 3;: Print "] [";: Print Using "a##b##c"; 1; 2; 3;: Print "]"
Print "7: [";: Print Using "& is ## years"; "Ann"; 30; "Bob"; 41;: Print "] [";: Print Using "& is ## years"; "Ann"; 30; "Bob";
Print "] [";: Print Using "head ## mid ## tail"; 1;: Print "] [";: Print Using "head ## mid _# tail"; 1;: Print "]"
Print "8: [";: Print Using "abc"; 1;: Print "] [";: Print Using "abc"; "x";: Print "] [";: Print Using "_#"; 1;: Print "]"
Print "9: ";: Print Using "##"; 1
Print Using "##"; 1;
Print "|";
Print Using "##"; 1,
Print "|"
Print "10: ";: Print Using "###"; 1;: Print Pos(0);: Print Using "###"; 1,: Print Pos(0)
f = "<##>"
Print "11: ";: Print Using f; 5;: Print Using f + f; 5; 6;: Print Using (f); 7;: Print Using String$(3, "#") + ".#"; 5
Print "12: ";: Print Using once$; 1; 2; 3
n = 7
Print "13: ";: Print Using "### ### ### &"; n + 1; Abs(-5); (n); Left$("abcd", 2)
Print "14: ";: Print Using "## ##"; 1; Tab(10); 2;: Print Using "## ##"; 1; Spc(3); 2;: Print Using "##"; Tab(40); 1
Print "15: ";: Print "a"; Using "##"; 5;: Print 1; 2; Using "##"; 5; 6;: Print "a"; "b"; Using "##"; 5;
Print Tab(60); Using "##"; 5;: Print ; Using "##"; 5
Print "16: ";: Print Using "## ## ##"; 1; a(9); 3: Print "| ";: Print Using "## & ##"; 1; Mid$("abc", 0); 3: Print "| ";
Print Using "## ##"; 1; Asc(""): Print "|"
Print "17: ";: Print Using "##"; 5;: Print Using "&"; " and a string"
System
handler:
Print "<error"; Err; ">";
Resume Next
Function once$
    Print "(format)";
    once$ = "#"
End Function
