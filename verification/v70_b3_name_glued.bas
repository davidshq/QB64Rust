$CONSOLE:ONLY
' Verification (study 44, section 2 item 6): a variable's name with `_` against it at a line end. Is `a_` the
' variable `a` and a continuation, or a name of its own?
a = 5
y = a_
+ 1
PRINT y
SYSTEM
