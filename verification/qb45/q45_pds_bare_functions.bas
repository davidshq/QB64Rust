' PDS 7.1 functions that a program can call with no argument. QB 4.5 and QB64pe
' have none of them, so they read each name as a variable: "" or 0. Does PDS 7.1
' read each bare name as its function? (the `pds-function` lint) Each line below
' prints -1 only if the name is a function: a variable would be "" or 0. The run
' folder holds P.BAS, P.OBJ, P.EXE and OUT.TXT, so a bare DIR$ after DIR$("*.*")
' gives a second name. SAVEPOINT is not here: it is an ISAM function, and without
' the ISAM program loaded it raises a run-time error, which waits for a key. With
' `x = SAVEPOINT` alone, BC compiled it and the run stopped there (2026-11-26).
OPEN "OUT.TXT" FOR OUTPUT AS #1
c$ = CURDIR$
PRINT #1, "CURDIR$ len>0:"; LEN(c$) > 0
PRINT #1, "CURDIR$ has colon:"; INSTR(c$, ":") = 2
d$ = DIR$("*.*")
PRINT #1, "DIR$(spec) len>0:"; LEN(d$) > 0
e$ = DIR$
PRINT #1, "DIR$ len>0:"; LEN(e$) > 0
PRINT #1, "DIR$ another name:"; e$ <> d$
PRINT #1, "STACK>0:"; STACK > 0
CLOSE #1
