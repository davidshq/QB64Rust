$CONSOLE:ONLY
' Verification (m2-parser-breadth, M7): a FOR opened in the main file and closed by NEXT in an included file.
FOR i = 1 TO 2
'$INCLUDE:'v16_inc/next.bi'
PRINT "after"
SYSTEM
