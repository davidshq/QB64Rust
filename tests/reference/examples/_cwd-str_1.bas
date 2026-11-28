$CONSOLE:ONLY
PRINT "_CWD$ is a folder:"; _DIREXISTS(_CWD$)
PRINT "Started in its own folder, it is the start folder:"; _CWD$ = _STARTDIR$
END
