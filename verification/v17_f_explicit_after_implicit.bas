$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT after an implicit variable, which is used again after it.
z = 1
OPTION _EXPLICIT
z = 2
PRINT z
SYSTEM
