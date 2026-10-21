' TEST: check-fail
$CONSOLE:ONLY
' Under OPTION _EXPLICIT, an undeclared variable after a declaration marked "not supported yet" gets no error: the
' declaration may declare it, so a real error could be false (follow-on rule, m2-parser-breadth design D10, which
' replaced m2-control-flow-slice 4.3). `TYPE cos`, a `TYPE` named like a built-in, starts it (a `TYPE` block inside
' an `IF` block did until m4-names-arrays compiled it, `CONST c = EXP(1)` until m3-constant-evaluation computed it,
' `CONST c$ = CHR$(65)` until m3-errorline-chr-const, `DIM m AS _MEM` until m3-mem compiled it, `DIM AS LONG v`
' until that form was compiled, m2-defxxx-common-arrays task 3.3: it now declares `v`); the real error before it
' stays.
OPTION _EXPLICIT
a = 1
DIM AS LONG v
v = 2
TYPE cos
    m AS LONG
END TYPE
u = 3
b = 4
SYSTEM
