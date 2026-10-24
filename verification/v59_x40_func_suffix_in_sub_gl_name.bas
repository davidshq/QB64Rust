$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x40_func_suffix_in_sub_gl_name)
' a call of SUB _GL with a suffix (_GL&).
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _GL&
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
