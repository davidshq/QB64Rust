' TEST: check-fail
$CONSOLE:ONLY
' Included-file errors (m2-parser-breadth task 8.1, design D9, measured M7): a missing file; a nested include found
' only in the main file's folder (not looked up there); a SUB from a file included inside a SUB; a file including
' itself without a guard (100 levels); a $IF closed in an included file. A block closed in an included file
' compiles (include_cross_ok.bas).
'$INCLUDE:'inc/missing.bi'
'$INCLUDE:'inc/sub/a.bi'
'$INCLUDE:'inc/self_loop.bi'
s
SUB s
    '$INCLUDE:'inc/subs.bm'
END SUB
' A $IF closed in an included file (accepted by the old compiler; "not supported yet" here, as a $IF an included
' file leaves open): one mark, and the NEXT after it still closes its FOR.
FOR j = 1 TO 2
$IF WIN THEN
'$INCLUDE:'inc/endif.bi'
NEXT
