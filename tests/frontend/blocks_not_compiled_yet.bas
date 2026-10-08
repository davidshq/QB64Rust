' TEST: check-fail
$CONSOLE:ONLY
' Spec compiler/pipeline, "Statement not compiled yet": the block parses into a DoBlock holding a SwapStmt, and the
' only diagnostic is one "not supported yet" error at SWAP (the example was a FOR loop until m2-control-flow-slice
' task 5.2 typed FOR, then SELECT CASE until m2-core-builtins task 6.1 compiled it)
a = 1: b = 2
DO: SWAP a, b: LOOP UNTIL a > 0
