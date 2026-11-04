' QB 4.5 (DIVERGENCES-QB45.md Q-007): STRING$(n, "") with n above 0, and with n = 0.
' QB64pe reads the first byte of the empty string and raises no error.
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "1.";
s$ = STRING$(3, "")
PRINT #1, LEN(s$)
PRINT #1, "2.";
s$ = STRING$(0, "")
PRINT #1, LEN(s$)
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]";
RESUME NEXT
