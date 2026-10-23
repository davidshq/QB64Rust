' TEST: typed
$CONSOLE:ONLY
' $CHECKING anywhere (m4-handlers-logging-checking task 3.1, design D3): each statement below $CHECKING:OFF is
' unchecked, in source order through a block (its header as at the header, a statement inside after an ON),
' a label, a FUNCTION defined below OFF (verification\v50_c_checking, source_ordering/checking_off), mixed case,
' and an included file that turns checking off for the lines after it.
PRINT "checked"
$CHECKING:OFF
PRINT "unchecked"
x = 1
IF x THEN
    PRINT "inside"
    $checking:on
    PRINT "inside, checked"
END IF
PRINT "after the block"
$CHECKING:OFF
lbl:
FOR i = 1 TO 2
    PRINT i
NEXT
$Checking:On
PRINT f
'$INCLUDE:'inc/checking_off.bi'
PRINT "after the include"
$CHECKING:ON
SYSTEM

$CHECKING:OFF
FUNCTION f
    f = 2
END FUNCTION
$CHECKING:ON
