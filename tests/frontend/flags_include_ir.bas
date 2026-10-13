' TEST: ir
' Precompiler flags across files (m2-console-screenhide task 6.1): the setting and a $IF on its flag stand in an
' included file. Every round parses the included file again with the flags of the round; the trees of the last
' round are the program.
$IF _CONSOLE_ = 2 THEN
    _ECHO "main: 2 above the include"
$END IF
'$INCLUDE:'inc/flags_part.bi'
$IF _EXPLICITARRAY_ THEN
    PRINT "main: explicitarray 1"
$END IF
SYSTEM
