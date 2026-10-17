$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): the four functions for an error in an included file, in a
' file it includes, in a SUB of an included file called from main, and in main after each include (the main
' file's line counted around the $INCLUDE lines). Included files in v43_inc\.
ON ERROR GOTO handler
PRINT "1."
'$INCLUDE:'v43_inc/raise1.bi'
ERROR 5
PRINT "2."
'$INCLUDE:'v43_inc/outer.bi'
ERROR 6
PRINT "3."
insub
ERROR 7
PRINT "4."
SYSTEM

handler:
PRINT "  error"; ERR; "ERL"; ERL; "_ERRORLINE"; _ERRORLINE; "_INCLERRORLINE"; _INCLERRORLINE; "file ["; _INCLERRORFILE$; "]"
RESUME NEXT

'$INCLUDE:'v43_inc/subs.bi'
