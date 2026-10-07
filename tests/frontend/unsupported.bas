' TEST: check-fail
$CONSOLE:ONLY
' Constructs outside the slice: one error per statement, then the next statement is checked
OPTION BASE 1
PRINT 2 ^ 3; LEN("x")
x = a(1)
DIM s AS STRING * 4
IF x THEN PRINT "y": PRINT 1 MOD 2; SQR(2)
PRINT "a" - "b"; 1 + "x"
x% = "s"
SWAP x, y
END
