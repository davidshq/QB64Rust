$CONSOLE:ONLY
OPEN "_fullpath_1.tmp" FOR OUTPUT AS #1: CLOSE #1
p$ = _FULLPATH$("_fullpath_1.tmp")
PRINT "The full path ends with the name: "; RIGHT$(p$, 15)
PRINT "It is longer than the name:"; LEN(p$) > 15
KILL "_fullpath_1.tmp"
END
