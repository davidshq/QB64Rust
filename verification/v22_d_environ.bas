$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "The rest", second half): COMMAND$ without arguments on the command line
' (bare, with index 0, 1, a large and a negative one); ENVIRON$ with a name the program set itself, an unknown
' name, another case, an index (1, 0, a large one, negative, a float), and the ENVIRON statement without =, with an
' empty value and set twice. Nothing printed depends on the machine.
ON ERROR GOTO h
DIM s AS STRING, n AS LONG, d AS DOUBLE
PRINT "COMMAND$ ["; COMMAND$; "]"
PRINT "COMMAND$(0) is the program:"; LEN(COMMAND$(0)) > 0
PRINT "COMMAND$(1) ["; COMMAND$(1); "]"
PRINT "COMMAND$(100) ["; COMMAND$(100); "]"
PRINT "COMMAND$(-1)": s = COMMAND$(-1): PRINT "["; s; "]"
d = 1.5: PRINT "COMMAND$(1.5) ["; COMMAND$(d); "]"
ENVIRON "V22_T=7": PRINT "set and read ["; ENVIRON$("V22_T"); "]"
PRINT "another case ["; ENVIRON$("v22_t"); "]"
s = "V22_T": PRINT "by a variable ["; ENVIRON$(s); "]"
PRINT "by an expression ["; ENVIRON$("V22" + "_T"); "]"
PRINT "unknown ["; ENVIRON$("V22_NO_SUCH_NAME"); "]"
PRINT "empty name ["; ENVIRON$(""); "]"
ENVIRON "V22_T=8": PRINT "set twice ["; ENVIRON$("V22_T"); "]"
ENVIRON "V22_T=": PRINT "set to nothing ["; ENVIRON$("V22_T"); "]"
ENVIRON "V22_U=a=b": PRINT "a value with = ["; ENVIRON$("V22_U"); "]"
ENVIRON "V22_V x y": PRINT "a blank for = ["; ENVIRON$("V22_V"); "]"
PRINT "without =:": ENVIRON "V22_W": PRINT "["; ENVIRON$("V22_W"); "]"
PRINT "ENVIRON$(1) has text:"; LEN(ENVIRON$(1)) > 0; INSTR(ENVIRON$(1), "=") > 0
n = 1: PRINT "by a LONG variable:"; LEN(ENVIRON$(n)) > 0
d = 1.4: PRINT "by a DOUBLE 1.4 is ENVIRON$(1):"; ENVIRON$(d) = ENVIRON$(1)
PRINT "ENVIRON$(0)": s = ENVIRON$(0): PRINT "["; s; "]"
PRINT "ENVIRON$(-1)": s = ENVIRON$(-1): PRINT "["; s; "]"
PRINT "ENVIRON$(100000) ["; ENVIRON$(100000); "]"
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
