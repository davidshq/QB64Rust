$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): precedence of NOT and _NEGATE against comparisons and AND, and of
' the logical operators among themselves. Each pair of readings gives different results.
PRINT "_NEGATE 0 = 5:"; _NEGATE 0 = 5
PRINT "NOT 0 = 5:"; NOT 0 = 5
PRINT "NOT 0 AND 1:"; NOT 0 AND 1
PRINT "_NEGATE 0 AND 2:"; _NEGATE 0 AND 2
PRINT "_NEGATE 0 + 5:"; _NEGATE 0 + 5
PRINT "NOT 0 + 5:"; NOT 0 + 5
PRINT "1 OR 2 XOR 3:"; 1 OR 2 XOR 3
PRINT "1 XOR 3 EQV 0:"; 1 XOR 3 EQV 0
PRINT "0 EQV 0 IMP 0:"; 0 EQV 0 IMP 0
PRINT "5 IMP 3 IMP 0:"; 5 IMP 3 IMP 0
PRINT "0 _ORELSE 1 _ANDALSO 0:"; 0 _ORELSE 1 _ANDALSO 0
PRINT "1 _ANDALSO 2 AND 4:"; 1 _ANDALSO 2 AND 4
PRINT "1 _ORELSE 0 OR 2:"; 0 _ORELSE 0 OR 2
PRINT "6 AND 3 = 2:"; 6 AND 3 = 2
PRINT "-2 MOD 3:"; -2 MOD 3
PRINT "- 7 \ 2 (unary minus first):"; -7 \ 2
PRINT "2 ^ -1 + 1:"; 2 ^ -1 + 1
SYSTEM
