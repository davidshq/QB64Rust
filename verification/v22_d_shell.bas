$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "The rest", second half): SHELL with a command (the order of its output
' and the program's), _HIDE and _DONTWAIT in either order, a command from a variable and an expression, an empty
' command, the SHELL function and its result type, a raising command. No command prints text that depends on the
' machine; a bare SHELL (an interactive shell) is only compiled, in v22_d_shell_bare.
ON ERROR GOTO h
DIM c AS STRING, n AS _INTEGER64, l AS LONG
PRINT "a": SHELL "echo b": PRINT "c"
c = "echo from a variable": SHELL c
SHELL "echo from " + "an expression"
PRINT "_HIDE:": SHELL _HIDE "echo hidden"
PRINT "_HIDE to a file:": SHELL _HIDE "echo to a file> v22_d_shell.tmp"
OPEN "v22_d_shell.tmp" FOR INPUT AS #1: LINE INPUT #1, c: CLOSE #1: KILL "v22_d_shell.tmp": PRINT "["; c; "]"
PRINT "_HIDE _DONTWAIT:": SHELL _HIDE _DONTWAIT "cmd /c exit 0"
PRINT "_DONTWAIT _HIDE:": SHELL _DONTWAIT _HIDE "cmd /c exit 0"
PRINT "an empty command:": SHELL _HIDE ""
PRINT "the function, exit 3:"; SHELL("cmd /c exit 3")
n = SHELL("cmd /c exit 0"): PRINT "the function, exit 0:"; n
PRINT "the function in arithmetic:"; SHELL("cmd /c exit 2") * 1000000000000
l = SHELL("echo from the function"): PRINT "its result:"; l
PRINT "exit 300:"; SHELL("cmd /c exit 300"); "exit -1:"; SHELL("cmd /c exit -1")
PRINT "a raising command:": SHELL CHR$(-1): PRINT "after"
PRINT "a raising command in the function:"; SHELL(CHR$(-1))
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
