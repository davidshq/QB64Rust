' TEST: check-fail
$CONSOLE:ONLY
' SHELL, COMMAND$, ENVIRON$ and the functions of task 7.4 rejected (verification\v22_x159-x177): the SHELL statement
' with a number; the SHELL function by its bare name, with a number, with two arguments; COMMAND$ with a string, two
' arguments, empty parentheses; ENVIRON$ by its bare name, with two arguments, empty parentheses; _ACOS with a
' string, with two arguments; _CEIL by its bare name; _STRCMP with a number, with one argument; _STARTDIR$ with
' empty parentheses, with an argument (v22_x186, x187). Left "not supported yet", though the old compiler accepts
' it (v22_x166): CALL SHELL(c$)
DIM s AS STRING, n AS LONG, d AS DOUBLE
SHELL 5
SHELL _HIDE n
n = SHELL
n = SHELL(5)
n = SHELL("a", "b")
s = COMMAND$("a")
s = COMMAND$(1, 2)
s = COMMAND$()
s = ENVIRON$
s = ENVIRON$("a", "b")
s = ENVIRON$()
d = _ACOS("a")
d = _ACOS(1, 2)
d = _CEIL
n = _STRCMP(1, "a")
n = _STRICMP("a")
s = _STARTDIR$()
s = _STARTDIR$(1)
CALL SHELL("echo x")
