$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (171_environ_two)
DIM s AS STRING
s = ENVIRON$("a", "b")
SYSTEM
