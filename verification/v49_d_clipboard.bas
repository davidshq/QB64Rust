$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _CLIPBOARD$ set and read: text, empty, NUL and extended characters, CR LF.
_CLIPBOARD$ = "abc": PRINT "["; _CLIPBOARD$; "]"; LEN(_CLIPBOARD$)
_CLIPBOARD$ = "": PRINT "["; _CLIPBOARD$; "]"; LEN(_CLIPBOARD$)
_CLIPBOARD$ = "a" + CHR$(0) + "b": PRINT LEN(_CLIPBOARD$)
_CLIPBOARD$ = CHR$(176) + CHR$(225) + "z": c$ = _CLIPBOARD$: PRINT LEN(c$); ASC(c$, 1); ASC(c$, 2); ASC(c$, 3)
_CLIPBOARD$ = "x" + CHR$(13) + CHR$(10) + "y": c$ = _CLIPBOARD$: PRINT LEN(c$)
DIM fs AS STRING * 3: fs = "q": _CLIPBOARD$ = fs: PRINT LEN(_CLIPBOARD$)
_clipboard$ = "lower": PRINT _clipboard$
_CLIPBOARD$ = ""
SYSTEM
