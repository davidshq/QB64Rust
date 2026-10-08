' TEST: ir
$CONSOLE:ONLY
' ON n GOTO / ON n GOSUB lowered (m2-core-builtins task 7.1, design D8): the value stored into a temporary, one
' Branch per label using the value even when it raised (GOTO: one statement), a Gosub per target with a jump to the
' end (GOSUB), then error 5 for a negative value; 0 and values past the count fall through
x = 2
ON x GOTO a, b
PRINT "next"
ON x GOSUB a, b
PRINT "back"
END
a: PRINT "a": RETURN
b: PRINT "b": RETURN
