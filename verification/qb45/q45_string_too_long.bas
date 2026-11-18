' Q-038 (DIVERGENCES-QB64PE.md D-067): what QB 4.5 raises when a concatenation would make a string longer than
' its limit of 32,767 bytes, and that the program goes on after it.
OPEN "OUT.TXT" FOR OUTPUT AS #1
ON ERROR GOTO trap
a$ = SPACE$(20000)
b$ = "x"
b$ = a$ + a$
PRINT #1, "after:"; LEN(a$); LEN(b$)
c$ = SPACE$(32767)
PRINT #1, "c:"; LEN(c$)
c$ = c$ + "y"
PRINT #1, "after c:"; LEN(c$)
CLOSE #1
END
trap:
PRINT #1, "error"; ERR
RESUME NEXT
