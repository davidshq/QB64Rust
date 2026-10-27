' QB 4.5 (row 4 of the crash-class rulings, study\39): QB64pe's form STATIC a(5) AS LONG.
' In QB 4.5 the number in STATIC a(n) counts dimensions, so a(1) is "Wrong number of dimensions".
DECLARE SUB s ()
DIM SHARED a(3) AS LONG
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
a(1) = 7
CALL s
PRINT #1, "main:"; a(1); UBOUND(a)
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]"
RESUME NEXT

SUB s
STATIC a(5) AS LONG
PRINT #1, "SUB:"; a(1); UBOUND(a)
END SUB
