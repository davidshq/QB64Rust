' TEST: ir
$CONSOLE:ONLY
' $RESIZE:SMOOTH and below it $RESIZE:OFF (m4-qb64pe-builtins design D3, task 4.1; verification\v53_x77): the last
' counts, so the window is neither resizable nor scaled.
$RESIZE:SMOOTH
$RESIZE:OFF
PRINT 1
