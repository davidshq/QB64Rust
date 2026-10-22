' TEST: cpp
$CONSOLE:ONLY
' The C++ of a line number (m4-line-numbers-whole-arrays, design D2): the label, `last_line=<n>;` with the old
' compiler's text, then its event check; below `$CHECKING:OFF` the label and `last_line` alone.
10 PRINT ERL
10.5 GOTO 30#
30# PRINT "x"
$CHECKING:OFF
40 PRINT "y"
$CHECKING:ON
SYSTEM
