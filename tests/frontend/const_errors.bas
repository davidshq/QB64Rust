' TEST: check-fail
$CONSOLE:ONLY
' CONST errors (m2-control-flow-slice D6, D10), each rejected by the old compiler too (verification\v17_e_*):
' a name used before its CONST line (plain, with a numeric suffix, in an earlier SUB), a variable then a CONST of
' its name, another value for the same constant, assignment to a constant, DIM of a constant's name (in main and
' in a SUB), a numeric constant used with `$`, a variable or a function outside the evaluator's list in the
' value, a string mixed with a number, a string comparison, a suffix of the wrong kind, `\` and MOD by 0, a
' negative number to a fractional power, `--5`
PRINT early
CONST early = 1
used% = 3
CONST used = 2
x = 1
CONST x = 2
CONST twice = 1
CONST twice = 2
CONST a = 1
a = 2
DIM a
PRINT a$
q = 4
CONST fromvar = q + 1
CONST fromlen = LEN("ab")
CONST mixed = "a" + 1
CONST strcmp = "a" < "b"
CONST n% = "x"
CONST s$ = 5
CONST idiv0 = 1 \ 0
CONST mod0 = 5 MOD 0
CONST negroot = (-8) ^ (1 / 3)
CONST dneg = --5
CONST insub = 3
sb
END

SUB early_use
    PRINT late
END SUB

CONST late = 7

SUB sb
    DIM insub
END SUB
