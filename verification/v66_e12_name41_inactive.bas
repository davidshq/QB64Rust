$CONSOLE:ONLY
' Verification (TODO.md, the 40-character limit on names): a name of 41 characters in a $IF branch not taken.
$LET X = 0
$IF X = 1 THEN
    abcdefghijabcdefghijabcdefghijabcdefghijk = 1
$END IF
PRINT "compiled"
SYSTEM
