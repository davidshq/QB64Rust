' TEST: check-fail
$CONSOLE:ONLY
' Line numbers, GOTO, GOSUB and RETURN parse, and sema marks each "not supported yet" (m2-parser-breadth task
' 5.3); a number after a label or a `:` is a real error, as in the old compiler (measured M3)
10 PRINT 1
GOTO 10
GOSUB 10
RETURN
lab: 20 PRINT 2
PRINT 3: 30
