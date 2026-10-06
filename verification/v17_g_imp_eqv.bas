$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): IMP and EQV with compound operands. The old compiler writes IMP as
' the text `~a | b` (v17_g_precedence: `5 IMP 3 IMP 0` prints 7); which left and right operands are affected?
' Each line prints the old result and, after the bar, the value computed by hand with NOT/OR/XOR.
x = 1: y = 2: z = 0
PRINT "1 OR 2 IMP 0:"; 1 OR 2 IMP 0; "|"; NOT (1 OR 2) OR 0
PRINT "(1 OR 2) IMP 0:"; (1 OR 2) IMP 0; "|"; NOT (1 OR 2) OR 0
PRINT "x OR y IMP z:"; x OR y IMP z; "|"; NOT (x OR y) OR z
PRINT "1 + 1 IMP 0:"; 1 + 1 IMP 0; "|"; NOT 2 OR 0
PRINT "x + y IMP z:"; x + y IMP z; "|"; NOT 3 OR 0
PRINT "NOT 0 IMP 0:"; NOT 0 IMP 0; "|"; NOT (NOT 0) OR 0
PRINT "1 IMP 2 OR 4:"; 1 IMP 2 OR 4; "|"; NOT 1 OR 6
PRINT "1 IMP 0 EQV 0:"; 1 IMP 0 EQV 0; "|"; NOT 1 OR (NOT (0 XOR 0))
PRINT "0 EQV 0 IMP 0:"; 0 EQV 0 IMP 0; "|"; NOT (NOT (0 XOR 0)) OR 0
PRINT "3 EQV 1 EQV 0:"; 3 EQV 1 EQV 0; "|"; NOT ((NOT (3 XOR 1)) XOR 0)
PRINT "1 OR 2 EQV 3:"; 1 OR 2 EQV 3; "|"; NOT (3 XOR 3)
PRINT "1 EQV 2 XOR 3:"; 1 EQV 2 XOR 3; "|"; NOT (1 XOR 1)
PRINT "x XOR y EQV z:"; x XOR y EQV z; "|"; NOT (3 XOR 0)
PRINT "1.5 IMP 0:"; 1.5 IMP 0; "|"; NOT 2 OR 0
PRINT "2.5 EQV 2:"; 2.5 EQV 2; "|"; NOT (2 XOR 2)
DIM i AS INTEGER
i = 5
PRINT "IMP type:"; (i IMP 0) * 1000000000; " EQV type:"; (i EQV 0) * 1000000000
PRINT "a > b IMP c:"; 3 > 2 IMP 0; "|"; NOT (-1) OR 0
PRINT "a IMP b > c:"; 0 IMP 3 > 2; "|"; NOT 0 OR -1
SYSTEM
