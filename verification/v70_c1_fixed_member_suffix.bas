$CONSOLE:ONLY
' Verification (study 44, section 2 item 2): a `$` suffix on a `STRING * n` member, as in DRAW's THEME.BI
' (`THEME.FILL_ADJ_ICON_FILE$ = ""` for a member `AS STRING * 64`). In a plain variable, an element and a nested
' member; read, written and passed to LEN.
TYPE t
    s AS STRING * 4
    n AS INTEGER
END TYPE
TYPE u
    inner AS t
END TYPE
DIM v AS t, w(2) AS t, q AS u
v.s$ = "abcdef"
PRINT "["; v.s$; "]"; LEN(v.s$)
v.s$ = "x"
PRINT "["; v.s$; "]"
w(1).s$ = "hi"
PRINT "["; w(1).s$; "]"
q.inner.s$ = "zz"
PRINT "["; q.inner.s$; "]"; LEN(q.inner.s)
SYSTEM
