' TEST: ir
$CONSOLE:ONLY
' STOP in the IR (m4-qb64pe-sources task 4.1; measured in verification\v52_x10-x19): alone, after THEN and ELSE,
' in a SUB and a FUNCTION, with a number, a FUNCTION's call and a _FLOAT expression with a name not declared (each
' checked, none evaluated: nothing of it is in the IR)
PRINT "a": STOP
IF x = 1 THEN STOP ELSE PRINT "b": STOP
STOP 3
STOP f(1)
STOP 1.5## + q
s
SUB s
    STOP
END SUB
FUNCTION f (n)
    IF n THEN f = 1 ELSE STOP
END FUNCTION
