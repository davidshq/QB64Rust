$Console:Only
' Operators (spec language/numeric-semantics, "Comparisons", "Logical operators", "Integer division and MOD",
' "Power"): results -1/0 typed LONG, float comparisons at the narrower type, string comparisons byte by byte (also
' above 127), logic in 32 bits for INTEGER/LONG and 64 with _INTEGER64, float operands rounded half to even (also
' for _ANDALSO, _ORELSE, _NEGATE), short circuit, \ and MOD signs and rounding, ^ left-associative with its result
' types, a negative base with a fractional exponent (error 5, placeholder 0), precedence, MOD by 0 (fatal, last).
' `x * 1000000000` tells a 32-bit computation (wraps) from a 64-bit one.
On Error GoTo h
Dim i As Integer, l As Long, a As _Integer64, b As _Integer64
Print "comparisons:"; 3 > 2; 2 > 3; "a" < "b"; 1 <> 1; 2 <= 2; 2 >= 3; 1 = 1
Print "compare type:"; (3 > 2) * 1000000000; (3 > 2) * 3000000000
v! = 2.1
Print "SINGLE against a literal:"; v! = 2.1; 2.1 = v!
s! = 0.1: d# = 0.1
Print "single vs double:"; s! = d#; d# = s!; s! < d#; s! = 0.1#; 0.1# > s!
Print "double vs float:"; d# = 0.1##
Print "int vs float:"; 1 = 1.0; 3 = 3.0000001
a = 9223372036854775807
b = -9223372036854775807 - 1
Print "int64 compare:"; a > a - 1; b < a; a = a; b <= b; a <> b
Print "string compare:"; "abc" = "abc"; "ab" < "abc"; "" < "a"; "B" < "a"; "abc" > "abd"; "a" <= "a"; "b" >= "a"
Print "high bytes:"; Chr$(200) > Chr$(100); Chr$(128) < "a"; Chr$(255) = Chr$(255)
Print "logic:"; 6 And 3; 6 Or 3; 6 Xor 3; Not 0; Not 5; 5 Eqv 3; 5 Imp 3
Print "float operands:"; 1.5 And 3; 2.5 Or 0; Not 1.5; 5 Xor 3; 5 Eqv 3; 5 Imp 3
Print "rounding:"; 2.5 And 7; 3.5 And 7; -2.5 Or 0; Not 2.5; Not -0.5
i = 5: l = 5
Print "NOT types:"; Not i; Not l; (Not i) * 1000000000; (Not l) * 1000000000
i = -32768
Print "NOT -32768%:"; Not i
Print "logic types:"; (i And i) * 1000000; (l And l) * 1000000000; (i Or 0) - 1
Print "int64 logic:"; a And 1; b Or 1; Not a; a Xor b; a Eqv b; a Imp 0
Print "LONG logic:"; 2147483647 And -1; &H7FFFFFFF Or &H80000000
Print "short circuit:"; 1 _AndAlso 2; 0 _OrElse 0; _Negate 0; _Negate 5
Print "short circuit rounded:"; 0.4 _AndAlso 1; _Negate 0.4; 5 _AndAlso 6; 0 _OrElse 2.5; _Negate -1
Print "short circuit calls:"; 0 _AndAlso f(1); 1 _OrElse f(2); 1 _AndAlso f(3); 0 _OrElse f(4)
Print "\ and MOD:"; 7 \ 2; -7 \ 2; 7.5 \ 2; -7 Mod 3; 7.5 Mod 2
Print "\ and MOD signs:"; 2.5 \ 1; 3.5 \ 1; -2.5 Mod 2; 7 Mod -3; -7 Mod -3; 7 \ -2
Print "int64 \ and MOD:"; a \ 2; a Mod 10; b \ 3; b Mod 7
l = 7
Print "LONG \ and MOD:"; l \ 2; l Mod 4; (l \ 1) * 1000000000
Print "power:"; 2 ^ 3 ^ 2; 2 ^ 0.5; (-2) ^ 3; (-2) ^ 2; 2 ^ -1; 0 ^ 0; 4 ^ 0.5
Print "power precedence:"; -2 ^ 2; 2 ^ -2 ^ 2
Print "power types:"; 2% ^ 0.5; 2& ^ 0.5; 2&& ^ 0.5; 2# ^ 0.5; 2! ^ 0.5
l = 3
Print "power LONG:"; l ^ 2; l ^ 0.5
w! = 5: w! = (-8) ^ (1 / 3)
Print "negative base:"; w!
Print "precedence:"; Not 1 = 2; 1 Or 2 And 0; 1 + 2 = 3; 3 = 1 + 2; 2 * 3 Mod 4; 7 \ 2 * 2; 5 Mod 3 \ 2
Print "chains:"; 1 < 2 < 3; 3 > 2 > 1; 1 = 1 = 1
Print "_NEGATE precedence:"; _Negate 0 And 1; _Negate 1 = 0
Print "string compare in an expression:"; ("b" > "a") + 1; ("a" + "b" = "ab") * 2
z = 0
Print "MOD by 0 is fatal"
Print 5 Mod z
Print "not printed"
System

h:
Print "[error"; Err; "]"
Resume Next

Function f (n)
    Print "[f"; n; "]";
    f = n
End Function
