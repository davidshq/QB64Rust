' TEST: check-fail
$CONSOLE:ONLY
' Jumps to a built-in's name (m4-names-implicit-arrays task 2.4): the old compiler's validlabel refuses every
' built-in's name, so each jump to one is "Invalid label!" or "Invalid label", whether a line of that name stands
' in the file or not, and a line named like a built-in function is "Syntax error" (measured, verification\v27_x63,
' v29_x78, x79, v55_x35 to x45). The lines between the errors are accepted.
DIM l AS LONG
ON ERROR GOTO BEEP
GOTO CLS
GOTO BEEP
GOTO abs
GOSUB BEEP
ON 1 GOTO BEEP
RESTORE BEEP
RESTORE abs
' Accepted: a label of another name.
GOTO done
' The name of a built-in statement and a colon is the statement; of a function, an error.
BEEP: l = 1
CLS: l = 2
abs: l = 3
done:
PRINT "ok"
SYSTEM
