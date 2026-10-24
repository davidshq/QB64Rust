' TEST: check-fail
$CONSOLE:ONLY
' Precompiler flags (m2-console-screenhide design D7, m4-statements-metacommands design D9;
' verification/v26_x27_flag_between, v26_x34_flag_between_const): a $IF or $ELSEIF that names _CONSOLE_ between
' $CONSOLE:ONLY and a later $CONSOLE. The old compiler's first pass reads it with 2, its main pass with 1: its
' program compiles nothing from the branches, and the CONST of the branch the first pass takes is "Name already in
' use" where it is used. A $IF below the second line (the flag is 1 there in both passes) and one on another flag
' are compiled as usual.
$IF _CONSOLE_ = 2 THEN
    CONST k = 2
$ELSEIF _CONSOLE_ = 1 THEN
    CONST k = 1
$ELSE
    CONST k = 0
$END IF
$IF _EXPLICIT_ THEN
    PRINT "explicit"
$END IF
$CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "below both: 1"
$END IF
PRINT k
SYSTEM
