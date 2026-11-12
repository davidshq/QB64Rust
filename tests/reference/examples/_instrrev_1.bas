$CONSOLE:ONLY
path$ = "games/napoleon/won1.bas"
p = _INSTRREV(path$, "/")
PRINT p; MID$(path$, p + 1)
PRINT _INSTRREV(10, path$, "/")
PRINT _INSTRREV(path$, "\\")
END
