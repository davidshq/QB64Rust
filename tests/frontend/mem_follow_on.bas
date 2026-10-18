' TEST: check-fail
$CONSOLE:ONLY
' The pipeline spec ("No follow-on errors after an unsupported declaration"), whose scenarios use a `TYPE` block
' inside a block (since m3-constant-evaluation, which computes `CONST c = EXP(1)`, the example until then;
' `CONST c$ = CHR$(65)` until m3-errorline-chr-const, `DIM m AS _MEM` until m3-mem): a real error before the
' declaration is reported; after it only "not supported yet" errors are (the string stored in a number variable
' gets none). The constants of QB64pe's auto-included file are all computed since m3-constant-evaluation
' (`auto_const_typed`).
n = "before"
IF n THEN
    TYPE tt
        m AS LONG
    END TYPE
END IF
n = "after"
SYSTEM
