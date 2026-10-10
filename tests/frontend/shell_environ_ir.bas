' TEST: ir
$CONSOLE:ONLY
' SHELL in the IR (m2-builtin-statements task 7.2, design D2): one built-in operation per form, naming the form
' matched (plain, _HIDE first, _DONTWAIT first), the second word or an absent slot, and the command or an absent
' slot; the SHELL function, COMMAND$ and ENVIRON$ are calls in expressions, COMMAND$ with an absent slot when bare,
' ENVIRON$ with a string or a LONG
DIM s AS STRING, t AS STRING, n AS LONG, q AS _INTEGER64
SHELL s
SHELL
SHELL _HIDE s + t
SHELL _HIDE _DONTWAIT s
SHELL _DONTWAIT
SHELL _DONTWAIT _HIDE "echo x"
q = SHELL(s)
s = COMMAND$
s = COMMAND$(n)
s = ENVIRON$(t)
s = ENVIRON$(n)
SUB p (c AS STRING)
    SHELL _HIDE c
END SUB
