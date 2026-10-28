' QB 4.5 (row 4 of the crash-class rulings, study\39): STATIC in a SUB beside a shared
' main-module array of the same name and type, in QB 4.5's own form (STATIC a(), then DIM once).
' Whether the SUB's array is its own, kept between calls, and the shared one untouched.
DECLARE SUB s ()
DIM SHARED a(3) AS LONG
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
a(1) = 7
CALL s
CALL s
PRINT #1, "main:"; a(1); UBOUND(a)
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]"
RESUME NEXT

SUB s
STATIC a() AS LONG, done AS INTEGER
IF done = 0 THEN DIM a(5) AS LONG: done = 1
PRINT #1, "SUB:"; a(1); UBOUND(a)
a(1) = a(1) + 1
END SUB
