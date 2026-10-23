$CONSOLE
$SCREENHIDE
' Verification (m4-memory-records, task 1.1): file GET and PUT of a whole array of a name that is no array
' (und(), und$(), unl&()), with data in the file, and _UCHARPOS of nd&() and of nd2&() after a use of nd2&(1).
DIM i AS INTEGER
_DEST _CONSOLE
f$ = "v56_e_undeclared_arrays.tmp"
OPEN f$ FOR BINARY AS #1
FOR i = 0 TO 15: PUT #1, , i: NEXT
GET #1, 1, und()
PRINT LBOUND(und); UBOUND(und); LEN(und())
FOR i = 0 TO 10: PRINT und(i);: NEXT: PRINT
und(3) = 1.5
PUT #1, 5, und()
PRINT LOF(1); SEEK(1)
GET #1, 5, i
PRINT i
GET #1, 1, unl&()
PRINT unl&(0); unl&(1); unl&(10); SEEK(1)
ON ERROR GOTO h
GET #1, 1, und$()
PRINT "und$"; LBOUND(und$); UBOUND(und$); "["; und$(0); "]"; SEEK(1)
CLOSE #1
KILL f$
SCREEN 12
n = _UCHARPOS("abc", nd&())
nd2&(1) = 7
n2 = _UCHARPOS("ab", nd2&())
_DEST _CONSOLE
PRINT n; LBOUND(nd&); UBOUND(nd&); nd&(0); nd&(1); nd&(2); nd&(3)
PRINT n2; nd2&(0); nd2&(1); nd2&(2)
SYSTEM
h:
PRINT "error"; ERR; "at line"; _ERRORLINE
RESUME NEXT
