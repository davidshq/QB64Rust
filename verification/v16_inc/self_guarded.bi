$IF SELFDONE = UNDEFINED THEN
$LET SELFDONE = 1
PRINT "self_guarded.bi: first level"
'$INCLUDE:'self_guarded.bi'
$END IF
PRINT "self_guarded.bi: end of a level"
