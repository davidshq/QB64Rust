' TEST: check-fail
$CONSOLE:ONLY
' Spec compiler/pipeline, "Statement not compiled yet": the loop parses into a ForBlock (parser snapshot `loops`)
' and the only diagnostic is one "not supported yet" error at FOR
FOR i = 1 TO 3: PRINT i: NEXT
