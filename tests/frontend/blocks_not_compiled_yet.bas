' TEST: check-fail
$CONSOLE:ONLY
' Spec compiler/pipeline, "Statement not compiled yet": the block parses into a DoBlock holding the FILES statement,
' and the only diagnostic is one "not supported yet" error at FILES (the example was a FOR loop until
' m2-control-flow-slice task 5.2 typed FOR, then SELECT CASE until m2-core-builtins task 6.1 compiled it, then SWAP
' until m2-builtin-statements task 5.3, then LSET until m3-console-files task 6.1, then STOP until m4-qb64pe-sources
' task 4.1, then RUN until m4-statements-metacommands task 5.1)
a = 1: b$ = "x"
DO: FILES: LOOP UNTIL a > 0
