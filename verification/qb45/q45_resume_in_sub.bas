' QB 4.5 (row 13 of the crash-class rulings, study\39): RESUME, RESUME NEXT and RESUME label
' written in a SUB that the main module's handler calls (QB64pe: verification\v57_x01-x04).
DECLARE SUB r ()
DECLARE SUB rn ()
DECLARE SUB rl ()
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
ERROR 5
PRINT #1, "after ERROR"
CLOSE #1
SYSTEM
h:
PRINT #1, "handler"; ERR
CALL r
CALL rn
CALL rl
PRINT #1, "after the calls"
RESUME NEXT

SUB r
PRINT #1, "in r"
RESUME
END SUB

SUB rl
RESUME back
back:
PRINT #1, "at back in rl"
END SUB

SUB rn
PRINT #1, "in rn"
RESUME NEXT
END SUB
