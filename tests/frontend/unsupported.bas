' TEST: check-fail
$CONSOLE:ONLY
' Constructs outside the slice: one error per statement; the marked DIM stands last (it drops later real errors, D10)
OPTION BASE 1
PRINT 2 ^ 3; RND(1)
x = a(1)
IF x THEN PRINT "y": PRINT 1 MOD 2; _MIN(2, 3)
PRINT "a" - "b"; 1 + "x"
x% = "s"
SWAP x, y
DIM s AS _MEM
END
