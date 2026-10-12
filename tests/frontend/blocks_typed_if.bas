' TEST: typed
$CONSOLE:ONLY
' IF as a typed block (m2-control-flow-slice task 5.2, design D3): the block form with ELSEIF and ELSE, nested;
' the single-line form, also with ELSE and as `IF c GOTO x`; conditions keep their own type (any number)
a = 1: b% = 2
IF a > 0 THEN
    PRINT "a"
ELSEIF b% THEN
    IF a THEN PRINT "nested" ELSE PRINT "no"
ELSEIF a + b% = 3 THEN
ELSE
    PRINT "else"
END IF
IF b% = 2 THEN PRINT "one": PRINT "two"
IF a GOTO done
PRINT "skipped"
done:
IF a <> 1 THEN
END IF
