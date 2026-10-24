' TEST: check-fail
$CONSOLE:ONLY
' Spec compiler/pipeline, "Statement not compiled yet": the block parses into a DoBlock holding the FOR block over
' an element of a member array (verification\v56_f3, left marked by m4-memory-records), and the only diagnostic is
' one "not supported yet" error at the FOR variable (the example was a FOR loop until m2-control-flow-slice task
' 5.2 typed FOR, then SELECT CASE until m2-core-builtins task 6.1 compiled it, then SWAP until
' m2-builtin-statements task 5.3, then LSET until m3-console-files task 6.1, then STOP until m4-qb64pe-sources task
' 4.1, then RUN until m4-statements-metacommands task 5.1, then FILES until m4-builtins-remaining task 6.1)
TYPE t
    m(1 TO 2) AS LONG
END TYPE
DIM r AS t
a = 1: b$ = "x"
DO: FOR r.m(1) = 1 TO 2: NEXT: LOOP UNTIL a > 0
