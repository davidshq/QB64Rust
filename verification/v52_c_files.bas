$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): _READFILE$, _WRITEFILE and _FULLPATH$: a round trip with a NUL,
' an empty file, a file replaced by a shorter one, a missing file, a folder read as a file, a write into a missing
' folder, _FULLPATH$ of ".", of a file, of a file with "/" and "..", of an empty string, of a missing path, under a
' handler that prints ERR and what was returned. Only what does not depend on the folder is printed.
ON ERROR GOTO h
f$ = "v52_c_files.tmp"
_WRITEFILE f$, "a" + CHR$(0) + "b"
d$ = _READFILE$(f$)
PRINT LEN(d$); ASC(d$, 2); RIGHT$(d$, 1)
_WRITEFILE f$, ""
d$ = "x": d$ = _READFILE$(f$)
PRINT "empty"; LEN(d$)
_WRITEFILE f$, "longer text"
_WRITEFILE f$, "ab"
PRINT "replaced"; LEN(_READFILE$(f$))
OPEN f$ FOR BINARY AS #1
PRINT "lof"; LOF(1)
CLOSE #1
PRINT "missing"
d$ = "x": d$ = _READFILE$("v52_c_nope.tmp"): PRINT "["; d$; "]"
PRINT "folder"
d$ = "x": d$ = _READFILE$("."): PRINT "["; d$; "]"
PRINT "write into a missing folder"
_WRITEFILE "v52_c_no_such_dir\x.tmp", "a"
PRINT _FILEEXISTS("v52_c_no_such_dir\x.tmp")
PRINT "fullpath"
p$ = _FULLPATH$(".")
PRINT p$ = _CWD$ + "\"; RIGHT$(p$, 1)
p$ = _FULLPATH$(f$)
PRINT p$ = _CWD$ + "\" + f$
p$ = _FULLPATH$("./" + f$)
PRINT p$ = _CWD$ + "\" + f$
q$ = _FULLPATH$("..")
PRINT LEFT$(_CWD$, LEN(q$) - 1) + "\" = q$; RIGHT$(q$, 1)
p$ = "x": p$ = _FULLPATH$(""): PRINT "empty ["; p$; "]"
p$ = "x": p$ = _FULLPATH$("v52_c_no\such\dir"): PRINT "missing ["; p$; "]"
p$ = "x": p$ = _FULLPATH$("v52_c_nope.tmp"): PRINT "missing file ["; p$; "]"
KILL f$
PRINT _FILEEXISTS(f$)
SYSTEM
h:
PRINT "ERR"; ERR
RESUME NEXT
