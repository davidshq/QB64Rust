' TEST: ir
' Precompiler flags (m2-console-screenhide task 6.1, design D7; verification/v26_x53_flag_three_rounds): three
' rounds. The only $CONSOLE stands inside $IF _EXPLICIT_, the only OPTION _EXPLICIT on the last line: the first
' round sets _EXPLICIT_, the second then meets $CONSOLE, the third reads every line with both flags 1.
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
