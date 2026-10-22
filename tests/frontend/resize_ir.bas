' TEST: ir
$CONSOLE:ONLY
' $RESIZE is a setting of the program (m4-qb64pe-builtins design D3, task 4.1; verification\v53_x72, x76-x80):
' each form, in any case of letters, in a $CONSOLE:ONLY program and in a SUB; the last in the file counts, here
' $resize:smooth in the SUB: resizable and smoothed.
$RESIZE:ON
$RESIZE:OFF
$Resize:Stretch
s
SYSTEM
SUB s
    $resize:smooth
END SUB
