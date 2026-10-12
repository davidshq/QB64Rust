$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D6): a name read with a numeric suffix (&) before a numeric CONST of that name.
PRINT "before:"; c1&
CONST c1 = 5
PRINT "after:"; c1; c1&
SYSTEM
