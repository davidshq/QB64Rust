$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Files"): EOF, LOF, LOC, SEEK (function and statement), FREEFILE,
' _FILEEXISTS, _DIREXISTS, _CWD$: results and their types (by printing an overflowing product), on an empty file, at
' each position, on an output file, on a closed number; NAME onto an existing file, RMDIR of a folder that is not
' empty, KILL with a wildcard.
ON ERROR GOTO h
DIM f AS STRING, a AS STRING, n AS LONG, d AS DOUBLE
f = "v22_b_f.tmp"
PRINT "FREEFILE twice without opening"; FREEFILE; FREEFILE
OPEN f FOR OUTPUT AS #1
PRINT "FREEFILE with 1 open"; FREEFILE
OPEN f + "2" FOR OUTPUT AS #3
PRINT "FREEFILE with 1 and 3 open"; FREEFILE
CLOSE #3: KILL f + "2"
PRINT "output file: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
PRINT #1, "abc"
PRINT #1, "defgh"
PRINT "after two lines: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
CLOSE #1
OPEN f FOR INPUT AS #1
PRINT "input file at the start: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
LINE INPUT #1, a
PRINT "after one line: EOF"; EOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
LINE INPUT #1, a
PRINT "after two lines: EOF"; EOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
SEEK #1, 1
PRINT "after SEEK #1, 1: EOF"; EOF(1); "SEEK"; SEEK(1)
SEEK 1, 6
LINE INPUT #1, a: PRINT "from position 6: ["; a; "]"
SEEK 1, 100
PRINT "after SEEK 1, 100: EOF"; EOF(1); "SEEK"; SEEK(1)
CLOSE #1
PRINT "result types (an INTEGER would overflow at 32768, a LONG at 2^31)"
OPEN f FOR INPUT AS #1
PRINT LOF(1) * 100000000; SEEK(1) * 3000000000; (LOC(1) + 1) * 3000000000; EOF(1) * 3000000000; FREEFILE * 3000000000
PRINT _FILEEXISTS(f) * 3000000000; _DIREXISTS(".") * 3000000000
n = 1: d = 1.4
PRINT "argument forms: variable, float variable, float literal"; EOF(n); EOF(d); EOF(1.4); LOF(n); LOC(d); SEEK(1.4)
CLOSE #1
PRINT "empty file"
OPEN f FOR OUTPUT AS #1: CLOSE #1
OPEN f FOR INPUT AS #1
PRINT "EOF"; EOF(1); "LOF"; LOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
CLOSE #1
PRINT "binary and random"
OPEN f FOR OUTPUT AS #1: PRINT #1, "0123456789";: CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "EOF"; EOF(1); "LOF"; LOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
SEEK #1, 11
PRINT "at 11: EOF"; EOF(1); "LOC"; LOC(1)
CLOSE #1
OPEN f FOR RANDOM AS #1 LEN = 4
PRINT "random LEN 4: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; LOC(1); "SEEK"; SEEK(1)
SEEK #1, 3
PRINT "at record 3: LOC"; LOC(1); "SEEK"; SEEK(1)
CLOSE #1
PRINT "on a number that is not open"
PRINT "EOF"; EOF(5)
PRINT "LOF"; LOF(5)
PRINT "LOC"; LOC(5)
PRINT "SEEK"; SEEK(5)
PRINT "EOF(0)"; EOF(0)
PRINT "EOF(-1)"; EOF(-1)
PRINT "raising argument"; EOF(ASC(""))
PRINT "_FILEEXISTS: file"; _FILEEXISTS(f); "missing"; _FILEEXISTS("v22_b_none.tmp"); "folder"; _FILEEXISTS("."); "empty"; _FILEEXISTS("")
PRINT "_DIREXISTS: folder"; _DIREXISTS("."); "missing"; _DIREXISTS("v22_b_nodir"); "file"; _DIREXISTS(f); "empty"; _DIREXISTS("")
PRINT "raising arguments"; _FILEEXISTS(CHR$(-1)); _DIREXISTS(CHR$(-1))
PRINT "_CWD$ is not empty:"; LEN(_CWD$) > 0
MKDIR "v22_b_cwd": CHDIR "v22_b_cwd"
PRINT "_CWD$ ends with the folder: "; RIGHT$(_CWD$, 9)
CHDIR "..": RMDIR "v22_b_cwd"
PRINT "NAME onto an existing file"
OPEN f + "2" FOR OUTPUT AS #1: CLOSE #1
NAME f AS f + "2"
PRINT "NAME of an open file"
OPEN f FOR INPUT AS #1
NAME f AS f + "3"
CLOSE #1
PRINT _FILEEXISTS(f); _FILEEXISTS(f + "3")
PRINT "KILL of an open file"
OPEN f + "2" FOR INPUT AS #1
KILL f + "2"
CLOSE #1
PRINT _FILEEXISTS(f + "2")
PRINT "RMDIR of a folder that is not empty"
MKDIR "v22_b_dir"
OPEN "v22_b_dir/a.tmp" FOR OUTPUT AS #1: CLOSE #1
OPEN "v22_b_dir/b.tmp" FOR OUTPUT AS #1: CLOSE #1
RMDIR "v22_b_dir"
PRINT "KILL of a folder"
KILL "v22_b_dir"
PRINT "KILL with a wildcard"
KILL "v22_b_dir/*.tmp"
PRINT _FILEEXISTS("v22_b_dir/a.tmp"); _FILEEXISTS("v22_b_dir/b.tmp")
PRINT "KILL with a wildcard matching nothing"
KILL "v22_b_dir/*.tmp"
RMDIR "v22_b_dir"
PRINT _DIREXISTS("v22_b_dir")
KILL "v22_b_f.tmp*"
PRINT _FILEEXISTS(f); _FILEEXISTS(f + "2"); _FILEEXISTS(f + "3")
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
