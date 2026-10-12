' TEST: check-fail
$CONSOLE:ONLY
' The forms of the console and window metacommands the old compiler rejects, and a word after $INCLUDEONCE,
' which it compares the same way (m2-console-screenhide design D2, task 2.1; verification\v26_x09-x14, x23,
' x29-x32, x35-x37, x40-x49, x58, x60, x61). Each is an error of the program, none "not supported yet"; the
' messages are new.
$CONSOLE:OFF
$CONSOLE junk
$CONSOLE ' a comment
$CONSOLE: x = 1
$CONSOLE:ONLY junk
$CONSOLE:ONLY ' a comment
$CONSOLE : ONLY
$CONSOLE:ONLYX
$INCLUDEONCE junk
$SCREENHIDE junk
$SCREENHIDE ' a comment
$SCREENHIDE: y = 2
$SCREENSHOW junk
' After a statement and a colon, any metacommand:
x = 1: $CONSOLE
x = 1: $CONSOLE:ONLY
y = 2: $SCREENHIDE
x = 1: $INCLUDEONCE
x = 1: $LET FOO = 1
x = 1: $IF 1 THEN
' A tab before it, any metacommand; the line is then no metacommand, so the $IF opens no level:
	$CONSOLE
	$CONSOLE:ONLY
	$SCREENHIDE
 	 $SCREENSHOW
	$INCLUDEONCE
	$LET FOO = 1
	$IF 1 THEN
