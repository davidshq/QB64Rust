$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): HEX$, OCT$ and _BIN$ of VARPTR, VARSEG and _SCREENEXISTS as the
' direct argument (the old compiler passes the width 0), beside the same values through a variable.
DIM i AS INTEGER, a(1 TO 10) AS INTEGER, t AS STRING, q AS _INTEGER64
PRINT HEX$(VARPTR(i)); " "; OCT$(VARPTR(i)); " "; _BIN$(VARPTR(i))
PRINT HEX$(VARSEG(i)); " "; OCT$(VARSEG(i)); " "; _BIN$(VARSEG(i))
PRINT HEX$(VARPTR(a(9))); " "; OCT$(VARPTR(a(9))); " "; _BIN$(VARPTR(a(9)))
PRINT HEX$(VARSEG(a(1)) - VARSEG(a(1))); " "; OCT$(VARSEG(a(1)) - VARSEG(a(1)))
PRINT HEX$(VARPTR(t)); " "; HEX$(VARSEG(t))
PRINT "["; HEX$(_SCREENEXISTS); "]["; OCT$(_SCREENEXISTS); "]["; _BIN$(_SCREENEXISTS); "]"
q = _SCREENEXISTS
PRINT "["; HEX$(q); "]["; OCT$(q); "]["; _BIN$(q); "]"
PRINT "["; HEX$(_SCREENEXISTS + 0); "]["; HEX$(-1&&); "]["; HEX$(q + 0); "]"
SYSTEM
