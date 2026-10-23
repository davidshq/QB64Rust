' TEST: cpp
' $RESIZE:STRETCH writes both lines of dyninfo.txt, which libqb reads (m4-qb64pe-builtins design D3, task 4.1;
' qb64pe.bas 13100-13106): ScreenResize=1; and ScreenResizeScale=1;.
$RESIZE:STRETCH
PRINT 1
