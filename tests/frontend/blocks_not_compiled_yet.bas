' TEST: check-fail
$CONSOLE:ONLY
' Spec compiler/pipeline, "Statement not compiled yet": the block parses into a SelectBlock (parser snapshot
' `select`) and the only diagnostic is one "not supported yet" error at SELECT (the example was a FOR loop until
' m2-control-flow-slice task 5.2 typed FOR)
x = 1
SELECT CASE x: CASE 1: PRINT x: END SELECT
