' TEST: check-fail
$CONSOLE:ONLY
' A tab after a console metacommand or $INCLUDEONCE: an error, as in the old compiler (verification\v26_x58,
' x59); blanks after it are accepted (v26_x15). The lines below end in a tab, in blanks, in a blank and a tab,
' and in a tab; then $ASSERTS and $DEBUG with a tab after them (m3-asserts-debug-zlib, verification\v33_x04,
' x12).
$SCREENHIDE	
$SCREENSHOW   
$CONSOLE 	
$INCLUDEONCE	
$ASSERTS	
$DEBUG	
PRINT "ok"
