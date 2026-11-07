' _CLIPBOARD$ = text, then _CLIPBOARD$ read: the text read back (tests/sandbox/README.md). The cases of
' verification\v49_d_clipboard and of the slice program s150_builtins_more, which race with the host's clipboard
' history there (STATUS.md, known bugs): text, empty, a NUL, extended characters, CR LF, fixed-length strings.
OPEN "out.txt" FOR OUTPUT AS #1
_CLIPBOARD$ = "abc": PRINT #1, "["; _CLIPBOARD$; "]"; LEN(_CLIPBOARD$)
_CLIPBOARD$ = "": PRINT #1, "["; _CLIPBOARD$; "]"; LEN(_CLIPBOARD$)
_CLIPBOARD$ = "a" + CHR$(0) + "b": PRINT #1, LEN(_CLIPBOARD$)
_CLIPBOARD$ = CHR$(176) + CHR$(225) + "z": c$ = _CLIPBOARD$: PRINT #1, LEN(c$); ASC(c$, 1); ASC(c$, 2); ASC(c$, 3)
_CLIPBOARD$ = "x" + CHR$(13) + CHR$(10) + "y": c$ = _CLIPBOARD$: PRINT #1, LEN(c$)
DIM fs AS STRING * 3: fs = "q": _CLIPBOARD$ = fs: PRINT #1, LEN(_CLIPBOARD$)
DIM f4 AS STRING * 4: f4 = "ab": _CLIPBOARD$ = f4: PRINT #1, LEN(_CLIPBOARD$)
_clipboard$ = "lower": PRINT #1, _clipboard$
_CLIPBOARD$ = ""
CLOSE #1
SYSTEM
