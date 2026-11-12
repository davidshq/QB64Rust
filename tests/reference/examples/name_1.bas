$CONSOLE:ONLY
OPEN "name_1.tmp" FOR OUTPUT AS #1: CLOSE #1
NAME "name_1.tmp" AS "name_1b.tmp"
PRINT _FILEEXISTS("name_1.tmp"); _FILEEXISTS("name_1b.tmp")
KILL "name_1b.tmp"
END
