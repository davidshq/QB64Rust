$CONSOLE:ONLY
' Verification (wasm-pointer-width, design D2 and D8): the values an _OFFSET holds where a pointer is W bytes.
' HEX$ and _BIN$ of -1 have the digits of W bytes; _MK$ and _CV, PUT and GET take W bytes; INPUT # stores the
' value; and an _OFFSET variable is passed by reference to an _INTEGER64 parameter only where both are 8 bytes
' (a store in the SUB reaches it there, and a copy is passed where a pointer is 4 bytes).
$IF 32BIT THEN
    CONST W = 4
$ELSE
    CONST W = 8
$END IF
DIM u AS _UNSIGNED _OFFSET, p AS _OFFSET, q AS _OFFSET, n AS _INTEGER64
u = -1
PRINT LEN(HEX$(u)) = 2 * W; LEN(_BIN$(u)) = 8 * W; LEFT$(HEX$(u), 4); u > 0
p = -2
PRINT p; HEX$(p) = STRING$(2 * W - 1, "F") + "E"
PRINT LEN(_MK$(_OFFSET, 5)) = W; _CV(_OFFSET, _MK$(_OFFSET, 123456))
OPEN "v73_d.tmp" FOR OUTPUT AS #1
PRINT #1, 4567
CLOSE #1
OPEN "v73_d.tmp" FOR INPUT AS #1
INPUT #1, q
CLOSE #1
PRINT q
KILL "v73_d.tmp"
OPEN "v73_d.tmp" FOR BINARY AS #1
p = 1234567
PUT #1, 1, p
PRINT LOF(1) = W
q = 0
GET #1, 1, q
PRINT q
CLOSE #1
KILL "v73_d.tmp"
p = 5
bump64 p
PRINT p = 5 + W \ 8
n = 7
bumpoff n
PRINT n = 7 + W \ 8
' A sum that stays below 2^31, so that it prints the same at both widths: past it, the 4-byte store would cut it.
p = 2000000000
p = p + 100000000
PRINT p > 0
SYSTEM

SUB bump64 (x AS _INTEGER64)
    x = x + 1
END SUB

SUB bumpoff (x AS _OFFSET)
    x = x + 1
END SUB
