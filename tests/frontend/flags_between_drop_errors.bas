' TEST: check-fail
$CONSOLE:ONLY
' A $IF between two settings of a flag (m4-statements-metacommands design D9): its branches are dropped, so a jump
' to a label or line number in one is "Label not defined" (verification/v57_j01_label_dropped_goto,
' v57_j02_line_dropped_goto, v57_j04_label_dropped_gosub_sub). A DO in the dropped part that crosses the $END IF is
' a parse error, and the drop ends with the DO: the error on the line after it is still reported.
GOTO here
GOSUB 10
$IF _CONSOLE_ = 2 THEN
here:
10 PRINT "2"
$ELSE
here:
10 PRINT "0"
$END IF
$IF _CONSOLE_ = 2 THEN
DO
$ELSE
DO
$END IF
$CONSOLE
    PRINT "loop"
LOOP UNTIL 1
a$ = 1
SYSTEM
