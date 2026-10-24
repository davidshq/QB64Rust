' TEST: check-fail
$CONSOLE:ONLY
' Constructs outside the slice: one error per statement; the marked declaration (a TYPE named like a built-in, since
' m4-names-arrays compiles the TYPE block inside a block that stood here, which replaced the CONST that
' m3-constant-evaluation computes) stands last (it drops later real errors, D10). The marks in expressions are
' VARPTR and VARSEG of a TYPE that holds a STRING in conventional memory (verification\v56_a5, left marked by
' m4-memory-records); _SETBIT and _TOGGLEBIT stood there until m4-builtins-remaining compiled them.
TYPE ts
    s AS STRING
END TYPE
DIM r AS ts
OPTION BASE 1
PRINT 2 ^ 3; VARPTR(r)
x = a(1)
IF x THEN PRINT "y": PRINT 1 MOD 2; VARSEG(r)
PRINT "a" - "b"; 1 + "x"
x% = "s"
RUN
TYPE cos
    m AS LONG
END TYPE
END
