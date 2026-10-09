' TEST: typed
$CONSOLE:ONLY
' Fixed-length strings (m2-numeric-types design D6, tasks 7.1 and 7.2): a `STRING * n` place (a variable by `AS`,
' by `CONST`, by the suffix `$n` or implicit, an element, a member) has the type `Str*n`; its value is a `Str` of its
' n bytes, so stores, comparisons and built-ins see plain strings. Passed to a STRING parameter it goes by
' reference, in parentheses too. `p$3` and `p$` are two variables. Same output as qb64pe.exe.
TYPE rec
    id AS LONG
    nm AS STRING * 6
END TYPE
CONST n = 3
DIM r AS rec, a(2) AS STRING * 3
r.nm = "bob": a(1) = "x"
PRINT "["; r.nm; "]"; LEN(r); "["; a(1); "]"; ASC(a(0), 1)
setlong r.nm
setlong (a(1))
PRINT "["; r.nm; "] ["; a(1); "]"
work
SYSTEM

SUB work
    DIM f AS STRING * 4, c AS STRING * n, p$3
    STATIC t AS STRING * 2
    f = "ab": c = "abcdef": p$3 = "xyz!": p$ = "plain": q$2 = "qqq": t = "stu"
    PRINT "["; f; "]"; LEN(f); f = "ab  "; "["; c; p$3; p$; q$2; t; "]"
    setlong (f)
    PRINT "["; f; "]"
END SUB

SUB setlong (s AS STRING)
    s = "longer text"
END SUB
