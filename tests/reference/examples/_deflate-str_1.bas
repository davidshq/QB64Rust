$CONSOLE:ONLY
s$ = STRING$(1000, "a")
c$ = _DEFLATE$(s$)
PRINT "1000 bytes deflated to"; LEN(c$)
PRINT "Inflated back:"; _INFLATE$(c$) = s$
PRINT "At level 0, stored:"; LEN(_DEFLATE$(s$, 0))
END
