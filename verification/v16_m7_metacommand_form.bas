$CONSOLE:ONLY
' Verification (m2-parser-breadth, M7): blanks around the colon of a comment $INCLUDE, and a comment $INCLUDE
' of a missing file inside an inactive $IF branch.
'$INCLUDE :  'v16_inc_b.bi'
$IF 0 THEN
'$INCLUDE:'v16_nothere.bi'
$END IF
SYSTEM
