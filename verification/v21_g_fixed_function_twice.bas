$CONSOLE:ONLY
' Verification (m2-numeric-types, task 8.2): two calls of a FUNCTION f$n in one expression. The result is a fixed
' string over bytes of the static pool that the FUNCTION releases on return, so the second call overwrites the
' first call's result before the + reads it (DIVERGENCES.md D-014: the new compiler returns a copy).
PRINT "["; fs$5("ab") + fs$5("cd"); "]"
x$ = fs$5("12") + fs$5("34"): PRINT "["; x$; "]"
PRINT "["; fs$5("ab"); fs$5("cd"); "]"
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
