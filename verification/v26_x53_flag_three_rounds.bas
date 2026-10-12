' Verification (step 11, task 1.4): three rounds. The only $CONSOLE stands inside $IF _EXPLICIT_, and the only
' OPTION _EXPLICIT on the last line. No output at all means no console.
$IF _EXPLICIT_ THEN
    $CONSOLE
$END IF
$SCREENHIDE
$IF _CONSOLE_ = 1 THEN
    _ECHO "console 1"
$END IF
$IF _CONSOLE_ = 1 AND _EXPLICIT_ = 1 THEN
    _ECHO "console 1 and explicit 1"
$END IF
SYSTEM
OPTION _EXPLICIT
