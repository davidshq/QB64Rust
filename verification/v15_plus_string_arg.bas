$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, review of task 3.2): unary + on a string argument; if it compiles, is the
' variable passed by reference (changed by the SUB)?
s$ = "abc"
t +s$
PRINT s$
SYSTEM
SUB t (q AS STRING)
    q = q + "!"
END SUB
