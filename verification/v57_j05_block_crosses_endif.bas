$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a DO opened in each branch of a $IF between two settings, closed after its $END IF.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
DO
$ELSEIF _CONSOLE_ = 1 THEN
DO
$ELSE
DO
$END IF
$CONSOLE
    PRINT "loop"
LOOP UNTIL 1
PRINT "after"
x = 1 +
SYSTEM
