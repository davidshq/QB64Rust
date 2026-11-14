$CONSOLE:ONLY
PRINT "_CWD$ is a folder:"; _DIREXISTS(_CWD$)
PRINT "At the start, it is the start folder:"; _CWD$ = _STARTDIR$
END
