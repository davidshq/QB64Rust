' TEST: check-fail
$CONSOLE:ONLY
' The forms of $CHECKING the old compiler rejects (m4-handlers-logging-checking task 3.1, design D3; its "Syntax
' error" in verification\v50_x07-x11, "Unexpected character on line" after a colon in x21).
$CHECKING
$CHECKING:
$CHECKING:MAYBE
$CHECKING: OFF
$CHECKING:OFF 'note
PRINT "a": $CHECKING:OFF
SYSTEM
