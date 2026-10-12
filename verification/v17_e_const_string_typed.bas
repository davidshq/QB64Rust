$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a string constant's use in string functions.
CONST s = "ab"
PRINT LEN(s); s + "c"; UCASE$(s)
SYSTEM
