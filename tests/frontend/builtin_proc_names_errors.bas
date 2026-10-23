' TEST: check-fail
$CONSOLE:ONLY
' Procedures named like a keyword or a built-in with a suffix (m4-names-implicit-arrays task 2.5): the old
' compiler's regid refuses a reserved word whatever the suffix, and a built-in FUNCTION's name unless the built-in
' must have `$` and the name has none ("Name already in use"); a SUB's name takes no suffix ("Type symbols after a
' SUB name are invalid"). Measured: verification\v42_x56, v52_x28, v55_x46 to x51.
PRINT "ok"
SYSTEM

FUNCTION exp$ (a$)
    exp$ = a$
END FUNCTION

FUNCTION len$ (a$)
    len$ = a$
END FUNCTION

FUNCTION abs% (a)
    abs% = a
END FUNCTION

FUNCTION timer# ()
    timer# = 1
END FUNCTION

SUB exp$
END SUB

SUB cls$
END SUB

SUB s& (n AS LONG)
END SUB

' Accepted: `CHR$` must have its `$`, so `chr` is free without it.
FUNCTION chr& (a)
    chr& = a
END FUNCTION
