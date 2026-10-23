$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _FILES$ of a folder, a pattern, no match, and bare before and after a listing.
ON ERROR GOTO h
CHDIR _STARTDIR$
MKDIR "v49b_dir"
CHDIR "v49b_dir"
FOR i = 1 TO 3
    OPEN "f" + LTRIM$(STR$(i)) + ".txt" FOR OUTPUT AS #1: CLOSE #1
NEXT
OPEN "g.dat" FOR OUTPUT AS #1: CLOSE #1
MKDIR "sub"
PRINT "bare first: ["; _FILES$; "]"
e$ = _FILES$("")
DO WHILE LEN(e$): PRINT "all: "; e$: e$ = _FILES$: LOOP
PRINT "after the end: ["; _FILES$; "]"
e$ = _FILES$("*.txt")
DO WHILE LEN(e$): PRINT "txt: "; e$: e$ = _FILES$: LOOP
e$ = _FILES$("*.*")
n = 0: DO WHILE LEN(e$): n = n + 1: e$ = _FILES$: LOOP: PRINT "*.*:"; n
e$ = _FILES$("nomatch*"): PRINT "no match: ["; e$; "]"
e$ = _FILES$("sub"): PRINT "folder: ["; e$; "]"; " next: ["; _FILES$; "]"
e$ = _FILES$("g.dat"): PRINT "file: ["; e$; "]"
e$ = _FILES$("f1.txt"): PRINT "first of two: ["; e$; "]"
e$ = _FILES$("*.dat"): PRINT "restarted: ["; e$; "]"; " ["; _FILES$; "]"
e$ = _FILES$("nodir/*"): PRINT "no folder: ["; e$; "]"
KILL "*.txt": KILL "g.dat": RMDIR "sub"
CHDIR ".."
RMDIR "v49b_dir"
PRINT "done"
SYSTEM
h:
PRINT "error"; ERR; "at"; _ERRORLINE
RESUME NEXT
