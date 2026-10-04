' TEST: check-fail
$CONSOLE:ONLY
' Constructs outside the slice: one error per statement, then the next statement is checked
FOR i = 1 TO 3
PRINT 2 ^ 3; LEN("x")
x = a(1)
DIM s AS STRING * 4
IF x THEN PRINT "y": PRINT 1 MOD 2
PRINT "a" - "b"; 1 + "x"
x% = "s"
END IF
END
