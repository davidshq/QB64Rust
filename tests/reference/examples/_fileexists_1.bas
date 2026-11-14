$CONSOLE:ONLY
OPEN "_fileexists_1.tmp" FOR OUTPUT AS #1: CLOSE #1
PRINT _FILEEXISTS("_fileexists_1.tmp"); _FILEEXISTS("no such file.tmp")
PRINT _DIREXISTS("."); _DIREXISTS("no such folder")
KILL "_fileexists_1.tmp"
END
