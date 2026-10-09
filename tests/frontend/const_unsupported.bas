' TEST: check-fail
$CONSOLE:ONLY
' CONST cases that are "not supported yet" (m2-control-flow-slice D6, D10): a function from the old evaluator's
' list, a value only `_FLOAT` precision can hold, a rounded value in another operation, `ROOT`, a string constant
' used with a number suffix, a CONST beside a variable of a SUB. (A suffix whose type cannot hold the value is
' supported since m2-numeric-types design D7: `const_suffixed.bas`; `/` by 0, a power beyond _INTEGER64 and a CONST
' after a label since its design D8: `decided_fixes_*.bas`.)
CONST sq = SQR(2)
CONST pi = _PI
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
