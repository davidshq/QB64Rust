' TEST: typed
$CONSOLE:ONLY
' change m2-parser-breadth, design D2: metacommand comments without a directive the compiler acts on
'$Format:Off
PRINT 1 '$FORMAT:ON
'$INCLUDEONCE
' plain comment with $INCLUDE:'x.bi' later in it
PRINT 2
