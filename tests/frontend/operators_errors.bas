' TEST: check-fail
$CONSOLE:ONLY
' Operator errors (m2-control-flow-slice D4; the old compiler's are verification\v17_g_err_*, v17_g_*_string):
' a string compared with a number, string operands of operators on numbers; an IMP whose left operand is an IMP
' (with or without parentheses) is not supported yet (the old compiler computes it as OR)
PRINT "a" = 1
PRINT 1 < a$
PRINT "a" AND "b"
PRINT "a" MOD "b"
PRINT "a" ^ 2
PRINT NOT "a"
PRINT _NEGATE a$
PRINT a$ _ANDALSO 1
PRINT 5 IMP 3 IMP 0
PRINT (5 IMP 3) IMP 0
PRINT 5 IMP (3 IMP 0)
END
