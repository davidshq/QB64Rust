' TEST: check-fail
$CONSOLE:ONLY
' OPTION BASE and $DYNAMIC (m2-defxxx-common-arrays task 3.2): the old compiler's rejections (verification\v24_x64
' to x68, x241, x242: only 0 or 1 in plain digits may follow BASE), a single negative bound and reversed bounds
' under base 1, an index below the base in a constant position is still a run-time matter (no error here), and what stays marked until dynamic
' arrays compile: a DIM below $DYNAMIC, by its own message, also after $STATIC and $DYNAMIC again
OPTION BASE 2
OPTION BASE 1.0
OPTION BASE 1&
OPTION BASE &H1
OPTION BASE
OPTION BASE n
CONST one = 1
OPTION BASE one
OPTION BASE 1
DIM neg(-1)
DIM rev(3 TO 1)
DIM ok(3)
ok(0) = 1
'$DYNAMIC
DIM d1(5)
'$STATIC
DIM s1(5)
REM $DYNAMIC
DIM d2(1 TO 5) AS LONG
SYSTEM
