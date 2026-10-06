$CONSOLE:ONLY
' Operators (spec language/numeric-semantics, "Comparisons", "Logical operators", "Integer division and MOD",
' "Power"): results -1/0 typed LONG, float comparisons at the narrower type, string comparisons byte by byte (also
' above 127), logic in 32 bits for INTEGER/LONG and 64 with _INTEGER64, float operands rounded half to even (also
' for _ANDALSO, _ORELSE, _NEGATE), short circuit, \ and MOD signs and rounding, ^ left-associative with its result
' types, a negative base with a fractional exponent (error 5, placeholder 0), precedence, MOD by 0 (fatal, last).
' `x * 1000000000` tells a 32-bit computation (wraps) from a 64-bit one.
ON ERROR GOTO h
DIM i AS INTEGER, l AS LONG, a AS _INTEGER64, b AS _INTEGER64
PRINT "comparisons:"; 3 > 2; 2 > 3; "a" < "b"; 1 <> 1; 2 <= 2; 2 >= 3; 1 = 1
PRINT "compare type:"; (3 > 2) * 1000000000; (3 > 2) * 3000000000
v! = 2.1
PRINT "SINGLE against a literal:"; v! = 2.1; 2.1 = v!
s! = 0.1: d# = 0.1
PRINT "single vs double:"; s! = d#; d# = s!; s! < d#; s! = 0.1#; 0.1# > s!
PRINT "double vs float:"; d# = 0.1##
PRINT "int vs float:"; 1 = 1.0; 3 = 3.0000001
a = 9223372036854775807
b = -9223372036854775807 - 1
PRINT "int64 compare:"; a > a - 1; b < a; a = a; b <= b; a <> b
PRINT "string compare:"; "abc" = "abc"; "ab" < "abc"; "" < "a"; "B" < "a"; "abc" > "abd"; "a" <= "a"; "b" >= "a"
PRINT "high bytes:"; CHR$(200) > CHR$(100); CHR$(128) < "a"; CHR$(255) = CHR$(255)
PRINT "logic:"; 6 AND 3; 6 OR 3; 6 XOR 3; NOT 0; NOT 5; 5 EQV 3; 5 IMP 3
PRINT "float operands:"; 1.5 AND 3; 2.5 OR 0; NOT 1.5; 5 XOR 3; 5 EQV 3; 5 IMP 3
PRINT "rounding:"; 2.5 AND 7; 3.5 AND 7; -2.5 OR 0; NOT 2.5; NOT -0.5
i = 5: l = 5
PRINT "NOT types:"; NOT i; NOT l; (NOT i) * 1000000000; (NOT l) * 1000000000
i = -32768
PRINT "NOT -32768%:"; NOT i
PRINT "logic types:"; (i AND i) * 1000000; (l AND l) * 1000000000; (i OR 0) - 1
PRINT "int64 logic:"; a AND 1; b OR 1; NOT a; a XOR b; a EQV b; a IMP 0
PRINT "LONG logic:"; 2147483647 AND -1; &H7FFFFFFF OR &H80000000
PRINT "short circuit:"; 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5
PRINT "short circuit rounded:"; 0.4 _ANDALSO 1; _NEGATE 0.4; 5 _ANDALSO 6; 0 _ORELSE 2.5; _NEGATE -1
PRINT "short circuit calls:"; 0 _ANDALSO f(1); 1 _ORELSE f(2); 1 _ANDALSO f(3); 0 _ORELSE f(4)
PRINT "\ and MOD:"; 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2
PRINT "\ and MOD signs:"; 2.5 \ 1; 3.5 \ 1; -2.5 MOD 2; 7 MOD -3; -7 MOD -3; 7 \ -2
PRINT "int64 \ and MOD:"; a \ 2; a MOD 10; b \ 3; b MOD 7
l = 7
PRINT "LONG \ and MOD:"; l \ 2; l MOD 4; (l \ 1) * 1000000000
PRINT "power:"; 2 ^ 3 ^ 2; 2 ^ 0.5; (-2) ^ 3; (-2) ^ 2; 2 ^ -1; 0 ^ 0; 4 ^ 0.5
PRINT "power precedence:"; -2 ^ 2; 2 ^ -2 ^ 2
PRINT "power types:"; 2% ^ 0.5; 2& ^ 0.5; 2&& ^ 0.5; 2# ^ 0.5; 2! ^ 0.5
l = 3
PRINT "power LONG:"; l ^ 2; l ^ 0.5
w! = 5: w! = (-8) ^ (1 / 3)
PRINT "negative base:"; w!
PRINT "precedence:"; NOT 1 = 2; 1 OR 2 AND 0; 1 + 2 = 3; 3 = 1 + 2; 2 * 3 MOD 4; 7 \ 2 * 2; 5 MOD 3 \ 2
PRINT "chains:"; 1 < 2 < 3; 3 > 2 > 1; 1 = 1 = 1
PRINT "_NEGATE precedence:"; _NEGATE 0 AND 1; _NEGATE 1 = 0
PRINT "string compare in an expression:"; ("b" > "a") + 1; ("a" + "b" = "ab") * 2
z = 0
PRINT "MOD by 0 is fatal"
PRINT 5 MOD z
PRINT "not printed"
SYSTEM

h:
PRINT "[error"; ERR; "]"
RESUME NEXT

FUNCTION f (n)
    PRINT "[f"; n; "]";
    f = n
END FUNCTION
