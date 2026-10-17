$CONSOLE:ONLY
' The error functions for an error in an included file (m3-errorline-chr-const; verification\v43_b_include): in
' a file, in a file it includes, in a SUB of an included file called from main, and in main after each include.
' _ERRORLINE there is the main file's line of the outermost $INCLUDE;
' _INCLERRORLINE and _INCLERRORFILE$ the line in the innermost file and its name. The included files are in this
' program's .files folder.
ON ERROR GOTO handler
PRINT "1."
'$INCLUDE:'inc/raise1.bi'
ERROR 5
PRINT "2."
'$INCLUDE:'inc/outer.bi'
ERROR 6
PRINT "3."
insub
ERROR 7
PRINT "4."
SYSTEM

handler:
PRINT "  error"; ERR; "_ERRORLINE"; _ERRORLINE; "_INCLERRORLINE"; _INCLERRORLINE; "file ["; _INCLERRORFILE$; "]"
RESUME NEXT

'$INCLUDE:'inc/subs.bi'
