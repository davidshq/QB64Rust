$CONSOLE:ONLY
' Slice (m4-builtins-param-types tasks 2.2, 3.3): _OS$, _MD5$, _CLIPBOARD$, _FILES$ and _CAST (verification/v49_a-d):
' the system's name by its parts, digests of strings of each kind, the clipboard set and read (up to a NUL),
' a folder listed by name, pattern and bare calls, and _CAST to each numeric type, truncated toward zero.
ON ERROR GOTO handler
DIM s AS STRING, f AS STRING * 4, e AS STRING, n AS LONG
DIM sg AS SINGLE, d AS DOUBLE, k AS LONG, u AS _UNSIGNED LONG, q AS _INTEGER64, b AS _BIT
PRINT INSTR(_OS$, "[WINDOWS]") > 0; LEN(_OS$) > 0
PRINT _MD5$("abc"); " "; _MD5$("")
f = "ab": s = "QB64": PRINT _MD5$(f); " "; _MD5$(s + s); LEN(_MD5$(STRING$(5000, 0)))
_CLIPBOARD$ = "slice": PRINT "["; _CLIPBOARD$; "]"
_CLIPBOARD$ = "a" + CHR$(0) + "b": PRINT LEN(_CLIPBOARD$)
_CLIPBOARD$ = f: PRINT LEN(_CLIPBOARD$)
_CLIPBOARD$ = "": PRINT "["; _CLIPBOARD$; "]"
e = _FILES$
MKDIR "s150_dir"
CHDIR "s150_dir"
FOR n = 1 TO 3
    OPEN "f" + LTRIM$(STR$(n)) + ".txt" FOR OUTPUT AS #1: CLOSE #1
NEXT
OPEN "g.dat" FOR OUTPUT AS #1: CLOSE #1
e = _FILES$("")
DO WHILE LEN(e): PRINT e; " ";: e = _FILES$: LOOP
PRINT
e = _FILES$("*.txt"): n = 0
DO WHILE LEN(e): n = n + 1: e = _FILES$: LOOP
PRINT n; "["; _FILES$("none*"); "]"; "["; _FILES$("g.dat"); "]"
KILL "*.txt": KILL "g.dat"
CHDIR ".."
RMDIR "s150_dir"
sg = -6.75: d = 7.5: k = 300: u = 4294967295: q = 9007199254740993: b = -1
PRINT _CAST(_BYTE, sg); _CAST(_BYTE, k); _CAST(_UNSIGNED _BYTE, u); _CAST(INTEGER, k * 300); _CAST(_UNSIGNED INTEGER, sg)
PRINT _CAST(LONG, q); _CAST(_UNSIGNED LONG, b); _CAST(_INTEGER64, d); _CAST(_UNSIGNED _INTEGER64, sg)
PRINT _CAST(_OFFSET, sg); _CAST(_UNSIGNED _OFFSET, k); _CAST(SINGLE, q); _CAST(DOUBLE, q); _CAST(_FLOAT, d / 3)
PRINT _CAST(INTEGER, 2.5); _CAST(INTEGER, -2.5); _CAST(LONG, 0.9999); HEX$(_CAST(_BYTE, -1)); " "; HEX$(_CAST(INTEGER, -1))
PRINT _CAST(_BYTE, 100) + _CAST(_BYTE, 100); _CAST(INTEGER, 30000) * 2; _CAST(_BYTE, d * 100); _CAST(LONG, d * 1E9)
k = _CAST(_UNSIGNED _BYTE, 511): PRINT k; STR$(_CAST(SINGLE, 2 / 3))
PRINT "done"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
