' TEST: check-fail
$CONSOLE:ONLY
' Spec compiler/pipeline, "Statement not compiled yet": the block parses into a DoBlock holding a StopStmt, and the
' only diagnostic is one "not supported yet" error at STOP (the example was a FOR loop until m2-control-flow-slice
' task 5.2 typed FOR, then SELECT CASE until m2-core-builtins task 6.1 compiled it, then SWAP until
' m2-builtin-statements task 5.3, then LSET until m3-console-files task 6.1)
a = 1: b$ = "x"
DO: STOP: LOOP UNTIL a > 0
