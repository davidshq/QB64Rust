$CONSOLE:ONLY
' SHELL, COMMAND$ and ENVIRON$ (spec language/builtin-statements "SHELL"; language/builtin-functions): SHELL with a
' command from a literal, a variable and an expression, between two PRINTs; _HIDE and _DONTWAIT in either order; an
' empty and a raising command; the SHELL function, an _INTEGER64, with the command's exit code; COMMAND$ bare and
' with an index (the program is run without arguments); ENVIRON$ by a name the program set itself, in another
' case, unknown, empty, and by an index; the ENVIRON statement setting a name twice, to nothing, with a blank for
' the =, without one; _STARTDIR$, which CHDIR does not change. Nothing printed depends on the machine.
ON ERROR GOTO h
DIM c AS STRING, s AS STRING, n AS LONG, q AS _INTEGER64, d AS DOUBLE
PRINT "a": SHELL "echo b": PRINT "c"
c = "echo from a variable": SHELL c
SHELL "echo from " + "an expression"
SHELL ("echo in parentheses")
PRINT "_HIDE:": SHELL _HIDE "echo hidden"
PRINT "_HIDE to a file:": SHELL _HIDE "echo to a file> s43_shell_environ.tmp"
OPEN "s43_shell_environ.tmp" FOR INPUT AS #1: LINE INPUT #1, c: CLOSE #1: KILL "s43_shell_environ.tmp"
PRINT "["; c; "]"
PRINT "_HIDE _DONTWAIT:": SHELL _HIDE _DONTWAIT "cmd /c exit 0"
PRINT "_DONTWAIT _HIDE:": SHELL _DONTWAIT _HIDE "cmd /c exit 0"
c = "cmd /c exit 0": PRINT "_DONTWAIT:": SHELL _DONTWAIT c
PRINT "an empty command:": SHELL _HIDE ""
PRINT "the function, exit 3:"; SHELL("cmd /c exit 3")
q = SHELL("cmd /c exit 0"): PRINT "the function, exit 0:"; q
PRINT "the function in arithmetic:"; SHELL("cmd /c exit 2") * 1000000000000
n = SHELL("echo from the function"): PRINT "its result:"; n
PRINT "exit 300:"; SHELL("cmd /c exit 300"); "exit -1:"; SHELL("cmd /c exit -1")
PRINT "a raising command:": SHELL CHR$(-1): PRINT "after"
PRINT "a raising command in the function:"; SHELL(CHR$(-1))
shellInSub "echo in a SUB"

PRINT "COMMAND$ ["; COMMAND$; "]"; LEN(COMMAND$)
PRINT "COMMAND$(0) is the program:"; LEN(COMMAND$(0)) > 0
PRINT "COMMAND$(1) ["; COMMAND$(1); "]"
PRINT "COMMAND$(100) ["; COMMAND$(100); "]"
PRINT "COMMAND$(-1)": s = COMMAND$(-1): PRINT "["; s; "]"
d = 1.5: PRINT "COMMAND$(1.5) ["; COMMAND$(d); "]"
n = 0: PRINT "COMMAND$(n) is COMMAND$(0):"; COMMAND$(n) = COMMAND$(0)

ENVIRON "S43_T=7": PRINT "set and read ["; ENVIRON$("S43_T"); "]"
PRINT "another case ["; ENVIRON$("s43_t"); "]"
s = "S43_T": PRINT "by a variable ["; ENVIRON$(s); "]"
PRINT "by an expression ["; ENVIRON$("S43" + "_T"); "]"
PRINT "unknown ["; ENVIRON$("S43_NO_SUCH_NAME"); "]"
PRINT "empty name ["; ENVIRON$(""); "]"
ENVIRON "S43_T=8": PRINT "set twice ["; ENVIRON$("S43_T"); "]"
ENVIRON "S43_T=": PRINT "set to nothing ["; ENVIRON$("S43_T"); "]"
ENVIRON "S43_U=a=b": PRINT "a value with = ["; ENVIRON$("S43_U"); "]"
ENVIRON "S43_V x y": PRINT "a blank for = ["; ENVIRON$("S43_V"); "]"
PRINT "without =:": ENVIRON "S43_W": PRINT "["; ENVIRON$("S43_W"); "]"
PRINT "ENVIRON$(1) has text:"; LEN(ENVIRON$(1)) > 0; INSTR(ENVIRON$(1), "=") > 0
n = 1: PRINT "by a LONG variable:"; ENVIRON$(n) = ENVIRON$(1)
d = 1.4: PRINT "by a DOUBLE 1.4 is ENVIRON$(1):"; ENVIRON$(d) = ENVIRON$(1)
d = 1.6: PRINT "by a DOUBLE 1.6 is ENVIRON$(2):"; ENVIRON$(d) = ENVIRON$(2)
PRINT "ENVIRON$(0)": s = ENVIRON$(0): PRINT "["; s; "]"
PRINT "ENVIRON$(-1)": s = ENVIRON$(-1): PRINT "["; s; "]"
PRINT "ENVIRON$(100000) ["; ENVIRON$(100000); "]"
PRINT "a child sees it:": ENVIRON "S43_T=child": SHELL "echo %S43_T%"

s = _STARTDIR$: PRINT "_STARTDIR$ is _CWD$ at the start:"; LEN(s) > 0; _STARTDIR$ = _CWD$
MKDIR "s43_shell_environ.dir": CHDIR "s43_shell_environ.dir"
PRINT "after CHDIR, unchanged:"; _STARTDIR$ = s; "and _CWD$ differs:"; _CWD$ <> s
CHDIR _STARTDIR$: RMDIR "s43_shell_environ.dir": PRINT "back:"; _CWD$ = s
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT

SUB shellInSub (c AS STRING)
    SHELL c
    SHELL _HIDE c + "> s43_shell_environ.tmp"
    PRINT "the file is there:"; _FILEEXISTS("s43_shell_environ.tmp")
    KILL "s43_shell_environ.tmp"
END SUB
