$CONSOLE:ONLY
' The file functions (spec language/file-io "File functions"; language/builtin-functions "Built-ins without
' parentheses"): EOF, LOF, LOC, SEEK and the SEEK statement on output, input, binary and random files, FREEFILE,
' _FILEEXISTS, _DIREXISTS, _CWD$. LOF, LOC and SEEK are computed in 64 bits and printed as LONG. Files are s38_*.tmp
' and removed again.
DIM f AS STRING, a AS STRING, n AS LONG, d AS DOUBLE, q AS _INTEGER64, big AS _INTEGER64
f = "s38_functions.tmp"
PRINT "FREEFILE twice without opening"; FREEFILE; FREEFILE
OPEN f FOR OUTPUT AS #1
PRINT "FREEFILE with 1 open"; FREEFILE
n = FREEFILE
OPEN f + "2" FOR OUTPUT AS n
PRINT "opened as"; n; "FREEFILE is now"; FREEFILE
CLOSE n: KILL f + "2"
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
n = 1: d = 2.5: q = 3
SEEK n, d
PRINT "SEEK n, 2.5:"; SEEK(1)
SEEK 1.4, 3.5
PRINT "SEEK 1.4, 3.5:"; SEEK(1)
SEEK 1, q
PRINT "SEEK 1, an _INTEGER64:"; SEEK(1)
SEEK 1, 100
PRINT "after SEEK 1, 100: EOF"; EOF(1); "SEEK"; SEEK(1)
big = 5000000000
SEEK 1, big
PRINT "a position beyond 32 bits is kept: "; SEEK(1) = big; SEEK(1) \ 1000000
SEEK 1, 5000000001
PRINT "and a literal one: "; SEEK(1) - big
PRINT "argument forms: a variable, a float variable, a float literal"; EOF(n); EOF(d - 1.4); EOF(1.4); LOF(n); LOC(1.2)
PRINT "64-bit arithmetic on LOF:"; LOF(1) * 1000000000 \ 1000; "printed as LONG:"; LOF(1) * 1000000000
big = LOF(1) * 1000000000
PRINT "stored into an _INTEGER64:"; big
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
OPEN "R", #1, f, 5
PRINT "old form, length 5: LOF"; LOF(1); "SEEK"; SEEK(1)
CLOSE
PRINT "_FILEEXISTS: file"; _FILEEXISTS(f); "missing"; _FILEEXISTS("s38_none.tmp"); "folder"; _FILEEXISTS("."); "empty"; _FILEEXISTS("")
PRINT "_DIREXISTS: folder"; _DIREXISTS("."); "missing"; _DIREXISTS("s38_nodir"); "file"; _DIREXISTS(f); "empty"; _DIREXISTS("")
PRINT "_CWD$ is not empty:"; LEN(_CWD$) > 0
MKDIR "s38_cwd": CHDIR "s38_cwd"
a = _CWD$
PRINT "_CWD$ names the folder it was changed to:"; INSTR(a, "s38_cwd") > 0
CHDIR "..": RMDIR "s38_cwd"
PRINT "NAME moves a file:";
NAME f AS f + "2"
PRINT _FILEEXISTS(f); _FILEEXISTS(f + "2")
KILL f + "2"
PRINT "KILL removed it:"; _FILEEXISTS(f + "2")
SYSTEM
