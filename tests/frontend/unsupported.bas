' TEST: check-fail
$CONSOLE:ONLY
' Constructs outside the slice: one error per statement; the marked CONST stands last (it drops later real errors, D10)
OPTION BASE 1
PRINT 2 ^ 3; _KEYDOWN(32)
x = a(1)
IF x THEN PRINT "y": PRINT 1 MOD 2; _KEYHIT
PRINT "a" - "b"; 1 + "x"
x% = "s"
STOP
CONST c$ = CHR$(65)
END
