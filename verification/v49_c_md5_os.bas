$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _MD5$ of several strings and the parts of _OS$.
PRINT _MD5$("abc")
PRINT _MD5$(CHR$(0)); LEN(_MD5$(CHR$(0)))
PRINT _MD5$(STRING$(1000, "x"))
PRINT _MD5$("")
s$ = "QB64": PRINT _md5$(s$ + s$)
DIM fs AS STRING * 5: fs = "ab": PRINT _MD5$(fs)
PRINT _OS$
PRINT INSTR(_OS$, "[WINDOWS]") > 0; INSTR(_OS$, "[64BIT]") > 0; INSTR(_OS$, "[LINUX]"); LEN(_OS$)
SYSTEM
