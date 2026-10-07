' TEST: check-fail
$CONSOLE:ONLY
' Line numbers parse, and sema marks them "not supported yet", also as the target of GOTO and GOSUB
' (m2-parser-breadth task 5.3; a plain RETURN is checked since m2-control-flow-slice task 5.1); a number after a
' label or a `:` is a real error, as in the old compiler (measured M3)
10 PRINT 1
GOTO 10
GOSUB 10
RETURN
lab: 20 PRINT 2
PRINT 3: 30
