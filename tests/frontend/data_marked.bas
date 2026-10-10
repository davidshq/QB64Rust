' TEST: check-fail
$CONSOLE:ONLY
' Forms of READ and RESTORE the old compiler accepts that are "not supported yet", each marked at the form
' (m2-builtin-statements task 4.3; verification\v22_c_restore, v22_x70, x84): RESTORE to a line number, a READ
' target in parentheses, a whole array as a target, a member of an element as a target
TYPE pt
    x AS LONG
END TYPE
DIM a(3) AS LONG, n AS LONG, va(2) AS pt
DATA 1, 2, 3, 4
RESTORE 100
READ (n)
READ a()
READ va(1).x
SYSTEM
