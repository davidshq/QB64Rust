$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a DO opened in a $IF whose passes agree, closed after its $END IF.
PRINT "before"
$IF WIN THEN
DO
$ELSE
DO
$END IF
    PRINT "loop"
LOOP UNTIL 1
PRINT "after"
SYSTEM
