' TEST: ir
$CONSOLE:ONLY
' IF lowered (m2-control-flow-slice task 6.2, design D8): the IF condition is a Skip branch to the next branch, an
' ELSEIF condition a UseValue branch (measured, verification\v17_b_elseif); a branch that ran jumps to the end,
' which carries the END IF line; without ELSE the last branch goes straight to the end. A single-line IF is the
' same, and IF c GOTO x is a branch over a jump to x
a = 2
IF a = 1 THEN
    PRINT "one"
ELSEIF CHR$(a) = "b" THEN
    PRINT "b"
ELSEIF a = 2 THEN
    PRINT "two"
ELSE
    PRINT "other"
END IF
IF a > 0 THEN
    PRINT "positive"
END IF
IF a THEN PRINT "yes" ELSE PRINT "no"
IF a = 2 GOTO done
IF a = 3 THEN GOTO done
PRINT "not reached"
done:
PRINT "done"
