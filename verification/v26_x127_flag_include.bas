' Verification (step 11, group 6): a setting and a $IF on its flag in an included file. The main file tests
' _CONSOLE_ above the include and _EXPLICITARRAY_ below it.
$IF _CONSOLE_ = 2 THEN
    _ECHO "main: 2 above the include"
$END IF
'$INCLUDE:'v26_inc/x127_part.bi'
$IF _EXPLICITARRAY_ THEN
    PRINT "main: explicitarray 1"
$END IF
SYSTEM
