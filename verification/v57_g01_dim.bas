$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a DIM in each branch, used after.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    DIM v AS STRING
$ELSEIF _CONSOLE_ = 1 THEN
    DIM v AS LONG
$ELSE
    DIM v AS DOUBLE
$END IF
$CONSOLE
PRINT "after"
v = 5
PRINT v
SYSTEM
