$CONSOLE:ONLY
' Slice (m3-console-files, task 4.3; from verification\v30_a_using_numbers): PRINT USING with numbers, 16 numbered
' lines, each value between brackets so that its blanks show. Digit fields and a field that is too small, the
' point and its rounding, the signs before and after, stars and dollars, the comma, the exponent with the letter of
' the value's type, the underscore and text around a field, a value of each numeric type (which runtime entry it
' goes to), a value that is not finite (error 5), and a format that is an expression.
ON ERROR GOTO handler
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, l AS LONG, ul AS _UNSIGNED LONG, q AS _INTEGER64
DIM u AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, v AS STRING
PRINT "1: [";: PRINT USING "#"; 5;: PRINT "] [";: PRINT USING "###"; 5;: PRINT "] [";: PRINT USING "###"; -5;
PRINT "] [";: PRINT USING "###"; 1234;: PRINT "] [";: PRINT USING "#"; -1;: PRINT "] [";: PRINT USING "###"; 0;: PRINT "]"
PRINT "2: [";: PRINT USING "##.##"; 3.14159;: PRINT "] [";: PRINT USING ".##"; 0.5;: PRINT "] [";: PRINT USING ".##"; 1.5;
PRINT "] [";: PRINT USING "##."; 12.4;: PRINT "] [";: PRINT USING "##.##"; 123.456;: PRINT "] [";: PRINT USING "#.##.##"; 1.5;: PRINT "]"
PRINT "3: [";: PRINT USING "#.#"; 0.25;: PRINT "] [";: PRINT USING "#.#"; 0.35;: PRINT "] [";: PRINT USING "##"; 0.5;
PRINT "] [";: PRINT USING "##"; 2.5;: PRINT "] [";: PRINT USING "#"; 9.5;: PRINT "] [";: PRINT USING "#.#"; -0.04;: PRINT "]"
PRINT "4: [";: PRINT USING "+###"; 5;: PRINT "] [";: PRINT USING "+###"; -5;: PRINT "] [";: PRINT USING "###+"; 5;
PRINT "] [";: PRINT USING "###-"; 5;: PRINT "] [";: PRINT USING "###-"; -5;: PRINT "] [";: PRINT USING "-###"; -5;: PRINT "]"
PRINT "5: [";: PRINT USING "**###"; 5;: PRINT "] [";: PRINT USING "$$###"; 5;: PRINT "] [";: PRINT USING "$$###"; -5;
PRINT "] [";: PRINT USING "**$###.##"; 12.5;: PRINT "] [";: PRINT USING "$###"; 5;: PRINT "] [";: PRINT USING "*###"; 5;: PRINT "]"
PRINT "6: [";: PRINT USING "#######,"; 1234567;: PRINT "] [";: PRINT USING "####,.##"; 1234.5;: PRINT "] [";
PRINT USING "#,###.##"; -1234.5;: PRINT "] [";: PRINT USING "####.##,"; 1234.5;: PRINT "] [";: PRINT USING "$$#,###.##"; 1234.5;: PRINT "]"
PRINT "7: [";: PRINT USING "##.##^^^^"; 1234.5;: PRINT "] [";: PRINT USING "##.##^^^^"; 0.00012345;: PRINT "] [";
PRINT USING "##.##^^^^^"; 1.5D+123;: PRINT "] [";: PRINT USING "##.##^^^"; 1234.5;: PRINT "] [";: PRINT USING "+#.##^^^^"; 1234.5;: PRINT "]"
s = 1234: d = 1234: f = 1234: l = 1234
PRINT "8: [";: PRINT USING "#.##^^^^"; s;: PRINT "] [";: PRINT USING "#.##^^^^"; d;: PRINT "] [";: PRINT USING "#.##^^^^"; f;
PRINT "] [";: PRINT USING "#.##^^^^"; l;: PRINT "] [";: PRINT USING "#.##^^^^"; 1234!;: PRINT "] [";: PRINT USING "#.##^^^^"; 1234#;: PRINT "]"
PRINT "9: [";: PRINT USING "_###"; 5;: PRINT "] [";: PRINT USING "a## b"; 5;: PRINT "] [";
PRINT USING "Total: ###.## units"; 12.5;: PRINT "] [";: PRINT USING "(##)"; -5;: PRINT "] [";: PRINT USING "##%"; 5;: PRINT "]"
i = -32768: l = 2147483647: b = -128: ub = 255: ul = 4294967295
PRINT "10: [";: PRINT USING "######"; i;: PRINT "] [";: PRINT USING "###########"; l;: PRINT "] [";: PRINT USING "####"; b;
PRINT "] [";: PRINT USING "####"; ub;: PRINT "] [";: PRINT USING "###########"; ul;: PRINT "]"
q = 9223372036854775807: u = 0: u = u - 1
PRINT "11: [";: PRINT USING "####################"; q;: PRINT "] [";: PRINT USING "#####################"; u;: PRINT "] [";
q = -q: PRINT USING "####################"; q;: PRINT "] [";: PRINT USING "###,###,###,###,###,###,###,"; u;: PRINT "]"
s = 1 / 3: d = 1 / 3: f = 1 / 3
PRINT "12: [";: PRINT USING "#.##########"; s;: PRINT "] [";: PRINT USING "#.####################"; d;: PRINT "] [";
PRINT USING "#.########################"; f;: PRINT "]"
s = 16777217: d = 1D+30
PRINT "13: [";: PRINT USING "##########"; s;: PRINT "] [";: PRINT USING "###"; d;: PRINT "] [";: PRINT USING "###"; i + 1;
PRINT "] [";: PRINT USING "###"; i / 4;: PRINT "] [";: PRINT USING "###"; LEN("abc");: PRINT "]"
d = 0: d = 1 / d
PRINT "14: [";: PRINT USING "###.##"; d;: PRINT "] [";: d = -d: PRINT USING "###.##"; d;: PRINT "] [";
d = 0: d = d / d: PRINT USING "###.##"; d;: PRINT "]"
v = "##"
PRINT "15: [";: PRINT USING v; 5;: PRINT "] [";: PRINT USING v + "." + v; 2.5;: PRINT "] [";: PRINT USING LEFT$("###xx", 3); 7;
PRINT "] [";: PRINT USING (v); 6;: PRINT "] [";: PRINT USING fmt$; 1; 2; 3;: PRINT "]"
PRINT "16: [";: PRINT USING "## ## ##"; 1; 2;: PRINT "] [";: PRINT USING "##x"; 1, 2, 3;: PRINT "] [";: PRINT USING "#"; 1; 2; 3;: PRINT "]"
SYSTEM
handler:
PRINT "<error"; ERR; ">";
RESUME NEXT
FUNCTION fmt$
    PRINT "(fmt)";
    fmt$ = "#."
END FUNCTION
