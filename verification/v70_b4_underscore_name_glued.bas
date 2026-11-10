$CONSOLE:ONLY
' Verification (study 44, section 2 item 6): a name that holds `_` (`a_b`) with another `_` against it at a line
' end, and a number with `_` against it.
a_b = 2
y = a_b_
+ 1
z = 1 + 2_
+ 3
PRINT y; z
SYSTEM
