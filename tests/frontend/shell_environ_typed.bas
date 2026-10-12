' TEST: typed
$CONSOLE:ONLY
' SHELL, COMMAND$ and ENVIRON$ in the typed tree (m2-builtin-statements task 7.2; measured in
' verification\v22_d_shell, v22_d_environ and from the C++): the SHELL function takes a string and is an _INTEGER64;
' COMMAND$ is called by its bare name or with an index converted to LONG; ENVIRON$ takes a string as it is or a
' number converted to LONG; the SHELL statement takes a string or nothing after the words _HIDE and _DONTWAIT in
' either order
DIM s AS STRING, t AS STRING, fx AS STRING * 8, n AS LONG, d AS DOUBLE, q AS _INTEGER64
q = SHELL("echo x")
q = SHELL(s + t)
n = SHELL(fx) * 2
s = COMMAND$
s = COMMAND$(1)
s = COMMAND$(d)
s = COMMAND$ + COMMAND$(n + 1)
s = ENVIRON$("PATH")
s = ENVIRON$(t)
s = ENVIRON$(fx)
s = ENVIRON$(1)
s = ENVIRON$(d)
s = ENVIRON$(n + 1)
PRINT SHELL("echo x"); COMMAND$; ENVIRON$(s)
SHELL "echo x"
SHELL s + t
SHELL (s)
SHELL
SHELL _HIDE s
SHELL _HIDE
SHELL _HIDE _DONTWAIT s
SHELL _DONTWAIT s
SHELL _DONTWAIT
SHELL _DONTWAIT _HIDE fx
