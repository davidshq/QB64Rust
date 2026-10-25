' TEST: check-fail
$CONSOLE:ONLY
' Constructs outside the slice: one error per statement; the marked declaration (a TYPE named like a built-in, since
' m4-names-arrays compiles the TYPE block inside a block that stood here, which replaced the CONST that
' m3-constant-evaluation computes) stands last (it drops later real errors, D10). The mark in an expression is
' VARSEG of an element of a _BIT array; _SETBIT and _TOGGLEBIT stood there until m4-builtins-remaining compiled
' them, VARSEG of a TYPE that holds a STRING until m4-left-marked.
TYPE ts
    s AS STRING
END TYPE
DIM r AS ts, bb(2) AS _BIT
OPTION BASE 1
PRINT 2 ^ 3; VARPTR(r)
x = a(1)
IF x THEN PRINT "y": PRINT 1 MOD 2; VARSEG(bb(1))
PRINT "a" - "b"; 1 + "x"
x% = "s"
RUN
TYPE cos
    m AS LONG
END TYPE
END
