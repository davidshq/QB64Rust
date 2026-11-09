' QB 4.5 (beside SOMEDAY's row on WRITE and a quote inside a string): does WRITE # escape a quote inside a string
' (CHR$(34))? The bytes of the file show what WRITE wrote; INPUT # then shows what reads back.
ON ERROR GOTO h
OPEN "W.TXT" FOR OUTPUT AS #2
a$ = "say " + CHR$(34) + "hi" + CHR$(34) + " now"
WRITE #2, a$, 7
CLOSE #2
OPEN "OUT.TXT" FOR OUTPUT AS #1
OPEN "W.TXT" FOR BINARY AS #2
b$ = SPACE$(LOF(2))
GET #2, 1, b$
CLOSE #2
PRINT #1, "1. bytes:";
FOR i = 1 TO LEN(b$)
    PRINT #1, ASC(MID$(b$, i, 1));
NEXT
PRINT #1, ""
PRINT #1, "2. text: "; b$
OPEN "W.TXT" FOR INPUT AS #2
PRINT #1, "3. INPUT # fields:";
DO WHILE NOT EOF(2)
    INPUT #2, f$
    PRINT #1, " ["; f$; "]";
LOOP
PRINT #1, ""
CLOSE
SYSTEM
h:
PRINT #1, " error"; ERR
RESUME NEXT
