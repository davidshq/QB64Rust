$CONSOLE:ONLY
' Verification (m2-builtin-statements task 7.4): _STARTDIR$, named by 23 upstream programs. It has no argument and
' is called bare; the folder the program was started in, which CHDIR does not change. Nothing printed depends on
' the machine (the program is started in its own folder).
ON ERROR GOTO h
DIM s AS STRING
PRINT "has text:"; LEN(_STARTDIR$) > 0
PRINT "is _CWD$ at the start:"; _STARTDIR$ = _CWD$
s = _STARTDIR$
MKDIR "v22_f_startdir.tmp": CHDIR "v22_f_startdir.tmp"
PRINT "after CHDIR, unchanged:"; _STARTDIR$ = s; "and _CWD$ differs:"; _CWD$ <> s
CHDIR _STARTDIR$: RMDIR "v22_f_startdir.tmp"
PRINT "back:"; _CWD$ = s
PRINT "in an expression:"; LEN(_STARTDIR$ + "x") = LEN(s) + 1
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
