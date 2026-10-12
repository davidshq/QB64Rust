$CONSOLE:ONLY
' Procedures named like built-ins (verification\v19_proc_names.txt): a SUB may take a built-in function's name
' (LOC, ABS, FRE), a FUNCTION a built-in statement's name (BEEP, WIDTH, CLOSE), both the bare name of a built-in
' written with `$` (LEFT, CHR). Called bare, with CALL and with arguments.
loc
CALL loc
abs -2.5
fre 1, "x"
PRINT beep; width&; close + 1
left
PRINT chr(65)
SYSTEM

SUB loc
    PRINT "loc"
END SUB

SUB abs (x)
    PRINT "abs"; x
END SUB

SUB fre (n AS LONG, s AS STRING)
    PRINT "fre"; n; s
END SUB

FUNCTION beep
    beep = 1
END FUNCTION

FUNCTION width&
    width& = 2
END FUNCTION

FUNCTION close
    close = 3
END FUNCTION

SUB left
    PRINT "left"
END SUB

FUNCTION chr (n AS LONG)
    chr = n + 1
END FUNCTION
