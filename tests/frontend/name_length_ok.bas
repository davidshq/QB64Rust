' TEST: check-ok
$CONSOLE:ONLY
' The 40-character limit on names (TODO.md row 1; measured, verification\v66_e08, e09, e12, e14): 40 letters and
' a suffix, a word of 41 in a comment, a string, DATA and a $IF branch not taken, and a dotted name of 40
' characters with its dot are not errors.
abcdefghijabcdefghijabcdefghijabcdefghij$ = "a"
abcdefghijabcdefghijabcdefghijabcdefghij&& = 1
' abcdefghijabcdefghijabcdefghijabcdefghijk
REM abcdefghijabcdefghijabcdefghijabcdefghijk
PRINT "abcdefghijabcdefghijabcdefghijabcdefghijk"
DATA abcdefghijabcdefghijabcdefghijabcdefghijk
$LET X = 0
$IF X = 1 THEN
    abcdefghijabcdefghijabcdefghijabcdefghijk = 1
$END IF
abcdefghijabcdefghij.abcdefghijabcdefghi = 2
