' QB 4.5 and PDS 7.1: LEFT$ and RIGHT$ with a negative length, and with 0.
' Civil War Battleset's file list reaches LEFT$(t$, -1) on an empty cell
' (its docs\devnotes.md, item 16). QB64pe gives "" there and raises no error.
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
a$ = "ABC"
PRINT #1, "1.";
s$ = LEFT$(a$, -1)
PRINT #1, LEN(s$)
PRINT #1, "2.";
s$ = RIGHT$(a$, -1)
PRINT #1, LEN(s$)
PRINT #1, "3.";
s$ = LEFT$(a$, 0)
PRINT #1, LEN(s$)
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]";
RESUME NEXT
