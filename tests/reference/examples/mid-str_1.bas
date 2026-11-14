$CONSOLE:ONLY
s$ = "Austerlitz"
PRINT MID$(s$, 4, 3)
PRINT MID$(s$, 7)
PRINT "["; MID$(s$, 20); "]"
MID$(s$, 1, 3) = "XYZ"
PRINT s$
MID$(s$, 9) = "ABCDEF"
PRINT s$; LEN(s$)
END
