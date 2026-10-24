$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x50_clear_two_args)
' _glClear, which takes one argument, with two.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glClear 1, 2
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
