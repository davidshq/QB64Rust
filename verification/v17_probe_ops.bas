$CONSOLE:ONLY
' Verification (m2-control-flow-slice, design Context, probe 3): operators, CONST, GOSUB in a SUB, RETURN
' without GOSUB in a SUB.
ON ERROR GOTO h
PRINT 3 > 2; 2 > 3; "a" < "b"
PRINT 1.5 AND 3; 2.5 OR 0; NOT 0; NOT 1.5; 5 XOR 3
PRINT 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2
PRINT 2 ^ 3 ^ 2; 5 EQV 3; 5 IMP 3
PRINT 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5
a! = 2.1: PRINT a! = 2.1; 2.1 = a!
PRINT 2 ^ 0.5
CONST c1 = 2 ^ 3 ^ 2, c2 = 7 \ 2, c3 = 1 / 3, c4 = "x" + "y", c5% = 3.7, c6 = 3 > 2
PRINT c1; c2; c3; c4; c5%; c6
s
s2
PRINT "end"
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT

SUB s
    GOSUB lab
    PRINT "s done"
    EXIT SUB
    lab:
    PRINT "lab in s"
    RETURN
END SUB

SUB s2
    RETURN
    PRINT "s2 after"
END SUB
