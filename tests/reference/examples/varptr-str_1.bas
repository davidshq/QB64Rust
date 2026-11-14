$CONSOLE:ONLY
DIM i AS INTEGER, s AS STRING
v$ = VARPTR$(i)
PRINT "VARPTR$ of an INTEGER:"; LEN(v$); "bytes, type"; ASC(v$, 1)
v$ = VARPTR$(s)
PRINT "VARPTR$ of a STRING:"; LEN(v$); "bytes, type"; ASC(v$, 1)
END
