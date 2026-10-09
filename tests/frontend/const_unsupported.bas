' TEST: check-fail
$CONSOLE:ONLY
' CONST cases that are "not supported yet" (m2-control-flow-slice D6, D10): a function from the old evaluator's
' list, `/` by 0 (the old compiler gives 0), a CONST after a label on the same line (the old compiler fails), an
' integer power beyond _INTEGER64 (the old evaluator wraps it), a value only `_FLOAT` precision can hold, a
' rounded value in another operation, `ROOT`, a string constant used with a number suffix, a CONST beside a
' variable of a SUB. (A suffix whose type cannot hold the value is supported since m2-numeric-types design D7:
' `const_suffixed.bas`.)
CONST sq = SQR(2)
CONST pi = _PI
CONST div0 = 1 / 0
lbl: CONST k = 4
CONST huge = 2 ^ 70
CONST third## = 1 / 3
CONST tenth = 0.1
CONST tenth3 = tenth * 3
CONST r = 8 ROOT 3
CONST st = "a"
PRINT st%
sb 1
END

SUB sb (p)
    v = 1
    CONST v = 2
    CONST p = 3
END SUB
