$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): operator corners. Integer-type tests use `x * 1000000000` style
' overflow or bit patterns; float typing shows in the printed digits.
ON ERROR GOTO h
DIM a AS _INTEGER64, b AS _INTEGER64
a = 9223372036854775807
b = -9223372036854775807 - 1
PRINT "int64 compare:"; a > a - 1; b < a; a = a; b <= b; a <> b
PRINT "int64 logic:"; a AND 1; b OR 1; NOT a; a XOR b; a EQV b; a IMP 0
PRINT "int64 idiv mod:"; a \ 2; a MOD 10; b \ 3; b MOD 7
PRINT "float beyond 2^63:"; 1E+19 \ 3; 9.3E+18 MOD 7; 1E+19 AND 1
PRINT "float rounding in logic:"; 2.5 AND 7; 3.5 AND 7; -2.5 OR 0; NOT 2.5; NOT -0.5
PRINT "idiv mod rounding:"; 2.5 \ 1; 3.5 \ 1; -2.5 MOD 2; 7 MOD -3; -7 MOD -3; 7 \ -2
DIM i AS INTEGER, l AS LONG
i = 5: l = 5
PRINT "NOT types:"; NOT i; NOT l; (NOT i) * 1000000000; (NOT l) * 1000000000
i = -32768
PRINT "NOT -32768%:"; NOT i
PRINT "logic types:"; (i AND i) * 1000000; (l AND l) * 1000000000; (i OR 0) - 1
PRINT "compare type:"; (3 > 2) * 1000000000; (3 > 2) * 3000000000
PRINT "power:"; (-2) ^ 3; (-2) ^ 2; 2 ^ -1; 0 ^ 0; (-8) ^ (1 / 3); 4 ^ 0.5
PRINT "power precedence:"; -2 ^ 2; 2 ^ -2 ^ 2
PRINT "power typing:"; 2 ^ 0.5; 2% ^ 0.5; 2& ^ 0.5; 2&& ^ 0.5; 2# ^ 0.5; 2! ^ 0.5
l = 3
PRINT "power LONG:"; l ^ 2; l ^ 0.5
PRINT "string compare high bytes:"; CHR$(200) > CHR$(100); CHR$(128) < "a"; CHR$(255) = CHR$(255)
PRINT "string compare:"; "abc" = "abc"; "ab" < "abc"; "" < "a"; "B" < "a"; "abc" > "abd"; "a" <= "a"; "b" >= "a"
s! = 0.1: d# = 0.1
PRINT "single vs double:"; s! = d#; d# = s!; s! < d#; s! = 0.1#; 0.1# > s!
PRINT "double vs float:"; d# = 0.1##
PRINT "int vs float:"; 1 = 1.0; 3 = 3.0000001; i = -32768!
PRINT "precedence:"; NOT 1 = 2; 1 OR 2 AND 0; 1 + 2 = 3; 3 = 1 + 2; 2 * 3 MOD 4; 7 \ 2 * 2; 5 MOD 3 \ 2
PRINT "chains:"; 1 < 2 < 3; 3 > 2 > 1; 1 = 1 = 1
PRINT "short circuit:"; 0 _ANDALSO f(1); 1 _ORELSE f(2); 1 _ANDALSO f(3); 0 _ORELSE f(4)
PRINT "short circuit values:"; 5 _ANDALSO 6; 0 _ORELSE 2.5; 0.4 _ANDALSO 1; _NEGATE 0.4; _NEGATE -1
PRINT "_NEGATE precedence:"; _NEGATE 0 AND 1; _NEGATE 1 = 0
PRINT "wrap:"; l * 0 + 2147483647 + 1; 32767 + 1
PRINT "logic on LONG wraps:"; 2147483647 AND -1; &H7FFFFFFF OR &H80000000
SYSTEM

h:
PRINT "[error"; ERR; "]";
RESUME NEXT

FUNCTION f (n)
    PRINT "[f"; n; "]";
    f = n
END FUNCTION
