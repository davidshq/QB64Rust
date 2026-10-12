' Verification (step 11, task 1.4): $ASSERTS:CONSOLE with $IF _CONSOLE_ above and below it. Nothing else gives
' the program a console, and _ECHO needs one.
$SCREENHIDE
$IF _CONSOLE_ = 1 THEN
    _ECHO "above: console 1"
$ELSE
    _ECHO "above: console not 1"
$END IF
$ASSERTS:CONSOLE
$IF _CONSOLE_ = 1 THEN
    _ECHO "below: console 1"
$ELSE
    _ECHO "below: console not 1"
$END IF
$IF _ASSERTS_ THEN
    _ECHO "asserts 1"
$ELSE
    _ECHO "asserts 0"
$END IF
SYSTEM
