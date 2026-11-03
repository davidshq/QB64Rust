' QB 4.5 (DIVERGENCES-QB45.md Q-005): the first bytes of a fixed-length string before any
' assignment: a variable, a TYPE member, an array element, and a local in a SUB on each of
' two calls. QB64pe gives 0 for each byte; an assignment pads with spaces (32) in both.
TYPE r
    m AS STRING * 3
END TYPE
DECLARE SUB codes (t$)
DECLARE SUB s ()
DIM f AS STRING * 4
DIM v AS r
DIM a(2) AS STRING * 2
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "variable:"; : codes f
PRINT #1, "member:"; : codes v.m
PRINT #1, "element:"; : codes a(1)
CALL s
CALL s
f = "ab"
PRINT #1, "assigned:"; : codes f
CLOSE #1
SYSTEM

SUB codes (t$)
FOR k = 1 TO LEN(t$)
    PRINT #1, ASC(MID$(t$, k, 1));
NEXT
PRINT #1,
END SUB

SUB s
DIM g AS STRING * 2
PRINT #1, "local:"; : codes g
g = "x"
END SUB
