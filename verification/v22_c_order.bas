$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Data"): the order of DATA items across the main module, an IF block
' that is not taken, an included file, the lines after SYSTEM, and procedures; bare DATA; DATA after a colon.
ON ERROR GOTO h
DATA m1, m2
IF 0 THEN
    DATA in_if
END IF
'$INCLUDE:'v22_inc/data.bi'
PRINT "x": DATA after_colon
DATA
DATA after_bare
DATA "q", , last
FOR i = 1 TO 20
    READ s$
    IF done THEN EXIT FOR
    PRINT i; "["; s$; "]"
NEXT
p
SYSTEM
DATA after_system
h:
PRINT "  error"; ERR
done = -1
RESUME NEXT

SUB p
    DATA in_sub
    PRINT "in p"
END SUB

FUNCTION f
    DATA in_function
END FUNCTION
