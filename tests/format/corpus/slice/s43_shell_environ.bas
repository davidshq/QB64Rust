$Console:Only
' SHELL, COMMAND$ and ENVIRON$ (spec language/builtin-statements "SHELL"; language/builtin-functions): SHELL with a
' command from a literal, a variable and an expression, between two PRINTs; _HIDE and _DONTWAIT in either order; an
' empty and a raising command; the SHELL function, an _INTEGER64, with the command's exit code; COMMAND$ bare and
' with an index (the program is run without arguments); ENVIRON$ by a name the program set itself, in another
' case, unknown, empty, and by an index; the ENVIRON statement setting a name twice, to nothing, with a blank for
' the =, without one; _STARTDIR$, which CHDIR does not change. Nothing printed depends on the machine.
On Error GoTo h
Dim c As String, s As String, n As Long, q As _Integer64, d As Double
Print "a": Shell "echo b": Print "c"
c = "echo from a variable": Shell c
Shell "echo from " + "an expression"
Shell ("echo in parentheses")
Print "_HIDE:": Shell _Hide "echo hidden"
Print "_HIDE to a file:": Shell _Hide "echo to a file> s43_shell_environ.tmp"
Open "s43_shell_environ.tmp" For Input As #1: Line Input #1, c: Close #1: Kill "s43_shell_environ.tmp"
Print "["; c; "]"
Print "_HIDE _DONTWAIT:": Shell _Hide _DontWait "cmd /c exit 0"
Print "_DONTWAIT _HIDE:": Shell _DontWait _Hide "cmd /c exit 0"
c = "cmd /c exit 0": Print "_DONTWAIT:": Shell _DontWait c
Print "an empty command:": Shell _Hide ""
Print "the function, exit 3:"; Shell("cmd /c exit 3")
q = Shell("cmd /c exit 0"): Print "the function, exit 0:"; q
Print "the function in arithmetic:"; Shell("cmd /c exit 2") * 1000000000000
n = Shell("echo from the function"): Print "its result:"; n
Print "exit 300:"; Shell("cmd /c exit 300"); "exit -1:"; Shell("cmd /c exit -1")
Print "a raising command:": Shell Chr$(-1): Print "after"
Print "a raising command in the function:"; Shell(Chr$(-1))
shellInSub "echo in a SUB"

Print "COMMAND$ ["; Command$; "]"; Len(Command$)
Print "COMMAND$(0) is the program:"; Len(Command$(0)) > 0
Print "COMMAND$(1) ["; Command$(1); "]"
Print "COMMAND$(100) ["; Command$(100); "]"
Print "COMMAND$(-1)": s = Command$(-1): Print "["; s; "]"
d = 1.5: Print "COMMAND$(1.5) ["; Command$(d); "]"
n = 0: Print "COMMAND$(n) is COMMAND$(0):"; Command$(n) = Command$(0)

Environ "S43_T=7": Print "set and read ["; Environ$("S43_T"); "]"
Print "another case ["; Environ$("s43_t"); "]"
s = "S43_T": Print "by a variable ["; Environ$(s); "]"
Print "by an expression ["; Environ$("S43" + "_T"); "]"
Print "unknown ["; Environ$("S43_NO_SUCH_NAME"); "]"
Print "empty name ["; Environ$(""); "]"
Environ "S43_T=8": Print "set twice ["; Environ$("S43_T"); "]"
Environ "S43_T=": Print "set to nothing ["; Environ$("S43_T"); "]"
Environ "S43_U=a=b": Print "a value with = ["; Environ$("S43_U"); "]"
Environ "S43_V x y": Print "a blank for = ["; Environ$("S43_V"); "]"
Print "without =:": Environ "S43_W": Print "["; Environ$("S43_W"); "]"
Print "ENVIRON$(1) has text:"; Len(Environ$(1)) > 0; InStr(Environ$(1), "=") > 0
n = 1: Print "by a LONG variable:"; Environ$(n) = Environ$(1)
d = 1.4: Print "by a DOUBLE 1.4 is ENVIRON$(1):"; Environ$(d) = Environ$(1)
d = 1.6: Print "by a DOUBLE 1.6 is ENVIRON$(2):"; Environ$(d) = Environ$(2)
Print "ENVIRON$(0)": s = Environ$(0): Print "["; s; "]"
Print "ENVIRON$(-1)": s = Environ$(-1): Print "["; s; "]"
Print "ENVIRON$(100000) ["; Environ$(100000); "]"
Print "a child sees it:": Environ "S43_T=child": Shell "echo %S43_T%"

s = _StartDir$: Print "_STARTDIR$ is _CWD$ at the start:"; Len(s) > 0; _StartDir$ = _CWD$
MkDir "s43_shell_environ.dir": ChDir "s43_shell_environ.dir"
Print "after CHDIR, unchanged:"; _StartDir$ = s; "and _CWD$ differs:"; _CWD$ <> s
ChDir _StartDir$: RmDir "s43_shell_environ.dir": Print "back:"; _CWD$ = s
Print "done"
System
h:
Print "  error"; Err
Resume Next

Sub shellInSub (c As String)
    Shell c
    Shell _Hide c + "> s43_shell_environ.tmp"
    Print "the file is there:"; _FileExists("s43_shell_environ.tmp")
    Kill "s43_shell_environ.tmp"
End Sub
