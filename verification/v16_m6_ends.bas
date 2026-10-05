$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $ENDIF as one word; nested $IF inside an inactive branch; garbage inside it.
$IF 0 THEN
this is (( not BASIC
$IF 1 THEN
also not BASIC ))
$ELSE
$END IF
SUB nested_in_inactive
$ENDIF
PRINT "after"
$IF 1 THEN
    PRINT "indented $IF body"
    $END IF
SYSTEM
