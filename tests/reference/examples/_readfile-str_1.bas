$CONSOLE:ONLY
_WRITEFILE "_readfile_1.tmp", "line one" + CHR$(10) + "line two"
d$ = _READFILE$("_readfile_1.tmp")
PRINT LEN(d$); "bytes:"
PRINT d$
KILL "_readfile_1.tmp"
END
