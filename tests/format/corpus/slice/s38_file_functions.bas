$Console:Only
' The file functions (spec language/file-io "File functions"; language/builtin-functions "Built-ins without
' parentheses"): EOF, LOF, LOC, SEEK and the SEEK statement on output, input, binary and random files, FREEFILE,
' _FILEEXISTS, _DIREXISTS, _CWD$. LOF, LOC and SEEK are computed in 64 bits and printed as LONG. Files are s38_*.tmp
' and removed again.
Dim f As String, a As String, n As Long, d As Double, q As _Integer64, big As _Integer64
f = "s38_functions.tmp"
Print "FREEFILE twice without opening"; FreeFile; FreeFile
Open f For Output As #1
Print "FREEFILE with 1 open"; FreeFile
n = FreeFile
Open f + "2" For Output As n
Print "opened as"; n; "FREEFILE is now"; FreeFile
Close n: Kill f + "2"
Print "output file: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Print #1, "abc"
Print #1, "defgh"
Print "after two lines: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Close #1
Open f For Input As #1
Print "input file at the start: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Line Input #1, a
Print "after one line: EOF"; EOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Line Input #1, a
Print "after two lines: EOF"; EOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Seek #1, 1
Print "after SEEK #1, 1: EOF"; EOF(1); "SEEK"; Seek(1)
Seek 1, 6
Line Input #1, a: Print "from position 6: ["; a; "]"
n = 1: d = 2.5: q = 3
Seek n, d
Print "SEEK n, 2.5:"; Seek(1)
Seek 1.4, 3.5
Print "SEEK 1.4, 3.5:"; Seek(1)
Seek 1, q
Print "SEEK 1, an _INTEGER64:"; Seek(1)
Seek 1, 100
Print "after SEEK 1, 100: EOF"; EOF(1); "SEEK"; Seek(1)
big = 5000000000
Seek 1, big
Print "a position beyond 32 bits is kept: "; Seek(1) = big; Seek(1) \ 1000000
Seek 1, 5000000001
Print "and a literal one: "; Seek(1) - big
Print "argument forms: a variable, a float variable, a float literal"; EOF(n); EOF(d - 1.4); EOF(1.4); LOF(n); Loc(1.2)
Print "64-bit arithmetic on LOF:"; LOF(1) * 1000000000 \ 1000; "printed as LONG:"; LOF(1) * 1000000000
big = LOF(1) * 1000000000
Print "stored into an _INTEGER64:"; big
Close #1
Print "empty file"
Open f For Output As #1: Close #1
Open f For Input As #1
Print "EOF"; EOF(1); "LOF"; LOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Close #1
Print "binary and random"
Open f For Output As #1: Print #1, "0123456789";: Close #1
Open f For Binary As #1
Print "EOF"; EOF(1); "LOF"; LOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Seek #1, 11
Print "at 11: EOF"; EOF(1); "LOC"; Loc(1)
Close #1
Open f For Random As #1 Len = 4
Print "random LEN 4: EOF"; EOF(1); "LOF"; LOF(1); "LOC"; Loc(1); "SEEK"; Seek(1)
Seek #1, 3
Print "at record 3: LOC"; Loc(1); "SEEK"; Seek(1)
Close #1
Open "R", #1, f, 5
Print "old form, length 5: LOF"; LOF(1); "SEEK"; Seek(1)
Close
Print "_FILEEXISTS: file"; _FileExists(f); "missing"; _FileExists("s38_none.tmp"); "folder"; _FileExists("."); "empty"; _FileExists("")
Print "_DIREXISTS: folder"; _DirExists("."); "missing"; _DirExists("s38_nodir"); "file"; _DirExists(f); "empty"; _DirExists("")
Print "_CWD$ is not empty:"; Len(_CWD$) > 0
MkDir "s38_cwd": ChDir "s38_cwd"
a = _CWD$
Print "_CWD$ names the folder it was changed to:"; InStr(a, "s38_cwd") > 0
ChDir "..": RmDir "s38_cwd"
Print "NAME moves a file:";
Name f As f + "2"
Print _FileExists(f); _FileExists(f + "2")
Kill f + "2"
Print "KILL removed it:"; _FileExists(f + "2")
System
