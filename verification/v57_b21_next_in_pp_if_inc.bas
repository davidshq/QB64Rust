$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): FOR in the main file, NEXT inside a $IF in an included file.
FOR i = 1 TO 2
'$INCLUDE:'v57_inc/next_in_if.bi'
PRINT "after"
SYSTEM
