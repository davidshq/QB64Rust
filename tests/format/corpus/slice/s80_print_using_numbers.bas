$Console:Only
' Slice (m3-console-files, task 4.3; from verification\v30_a_using_numbers): PRINT USING with numbers, 16 numbered
' lines, each value between brackets so that its blanks show. Digit fields and a field that is too small, the
' point and its rounding, the signs before and after, stars and dollars, the comma, the exponent with the letter of
' the value's type, the underscore and text around a field, a value of each numeric type (which runtime entry it
' goes to), a value that is not finite (error 5), and a format that is an expression.
On Error GoTo handler
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, l As Long, ul As _Unsigned Long, q As _Integer64
Dim u As _Unsigned _Integer64, s As Single, d As Double, f As _Float, v As String
Print "1: [";: Print Using "#"; 5;: Print "] [";: Print Using "###"; 5;: Print "] [";: Print Using "###"; -5;
Print "] [";: Print Using "###"; 1234;: Print "] [";: Print Using "#"; -1;: Print "] [";: Print Using "###"; 0;: Print "]"
Print "2: [";: Print Using "##.##"; 3.14159;: Print "] [";: Print Using ".##"; 0.5;: Print "] [";: Print Using ".##"; 1.5;
Print "] [";: Print Using "##."; 12.4;: Print "] [";: Print Using "##.##"; 123.456;: Print "] [";: Print Using "#.##.##"; 1.5;: Print "]"
Print "3: [";: Print Using "#.#"; 0.25;: Print "] [";: Print Using "#.#"; 0.35;: Print "] [";: Print Using "##"; 0.5;
Print "] [";: Print Using "##"; 2.5;: Print "] [";: Print Using "#"; 9.5;: Print "] [";: Print Using "#.#"; -0.04;: Print "]"
Print "4: [";: Print Using "+###"; 5;: Print "] [";: Print Using "+###"; -5;: Print "] [";: Print Using "###+"; 5;
Print "] [";: Print Using "###-"; 5;: Print "] [";: Print Using "###-"; -5;: Print "] [";: Print Using "-###"; -5;: Print "]"
Print "5: [";: Print Using "**###"; 5;: Print "] [";: Print Using "$$###"; 5;: Print "] [";: Print Using "$$###"; -5;
Print "] [";: Print Using "**$###.##"; 12.5;: Print "] [";: Print Using "$###"; 5;: Print "] [";: Print Using "*###"; 5;: Print "]"
Print "6: [";: Print Using "#######,"; 1234567;: Print "] [";: Print Using "####,.##"; 1234.5;: Print "] [";
Print Using "#,###.##"; -1234.5;: Print "] [";: Print Using "####.##,"; 1234.5;: Print "] [";: Print Using "$$#,###.##"; 1234.5;: Print "]"
Print "7: [";: Print Using "##.##^^^^"; 1234.5;: Print "] [";: Print Using "##.##^^^^"; 0.00012345;: Print "] [";
Print Using "##.##^^^^^"; 1.5D+123;: Print "] [";: Print Using "##.##^^^"; 1234.5;: Print "] [";: Print Using "+#.##^^^^"; 1234.5;: Print "]"
s = 1234: d = 1234: f = 1234: l = 1234
Print "8: [";: Print Using "#.##^^^^"; s;: Print "] [";: Print Using "#.##^^^^"; d;: Print "] [";: Print Using "#.##^^^^"; f;
Print "] [";: Print Using "#.##^^^^"; l;: Print "] [";: Print Using "#.##^^^^"; 1234!;: Print "] [";: Print Using "#.##^^^^"; 1234#;: Print "]"
Print "9: [";: Print Using "_###"; 5;: Print "] [";: Print Using "a## b"; 5;: Print "] [";
Print Using "Total: ###.## units"; 12.5;: Print "] [";: Print Using "(##)"; -5;: Print "] [";: Print Using "##%"; 5;: Print "]"
i = -32768: l = 2147483647: b = -128: ub = 255: ul = 4294967295
Print "10: [";: Print Using "######"; i;: Print "] [";: Print Using "###########"; l;: Print "] [";: Print Using "####"; b;
Print "] [";: Print Using "####"; ub;: Print "] [";: Print Using "###########"; ul;: Print "]"
q = 9223372036854775807: u = 0: u = u - 1
Print "11: [";: Print Using "####################"; q;: Print "] [";: Print Using "#####################"; u;: Print "] [";
q = -q: Print Using "####################"; q;: Print "] [";: Print Using "###,###,###,###,###,###,###,"; u;: Print "]"
s = 1 / 3: d = 1 / 3: f = 1 / 3
Print "12: [";: Print Using "#.##########"; s;: Print "] [";: Print Using "#.####################"; d;: Print "] [";
Print Using "#.########################"; f;: Print "]"
s = 16777217: d = 1D+30
Print "13: [";: Print Using "##########"; s;: Print "] [";: Print Using "###"; d;: Print "] [";: Print Using "###"; i + 1;
Print "] [";: Print Using "###"; i / 4;: Print "] [";: Print Using "###"; Len("abc");: Print "]"
d = 0: d = 1 / d
Print "14: [";: Print Using "###.##"; d;: Print "] [";: d = -d: Print Using "###.##"; d;: Print "] [";
d = 0: d = d / d: Print Using "###.##"; d;: Print "]"
v = "##"
Print "15: [";: Print Using v; 5;: Print "] [";: Print Using v + "." + v; 2.5;: Print "] [";: Print Using Left$("###xx", 3); 7;
Print "] [";: Print Using (v); 6;: Print "] [";: Print Using fmt$; 1; 2; 3;: Print "]"
Print "16: [";: Print Using "## ## ##"; 1; 2;: Print "] [";: Print Using "##x"; 1, 2, 3;: Print "] [";: Print Using "#"; 1; 2; 3;: Print "]"
System
handler:
Print "<error"; Err; ">";
Resume Next
Function fmt$
    Print "(fmt)";
    fmt$ = "#."
End Function
