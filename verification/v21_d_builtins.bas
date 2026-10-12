$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.4): the special-cased built-ins with arguments of the new numeric
' types: STR$, HEX$, OCT$, _BIN$, ABS, SGN, INT, FIX, CINT, CLNG, _ROUND, VAL(..., type), the result type of
' SQR and EXP (shown by the digits printed), CSNG, CDBL, and a LONG slot (LEFT$, CHR$). Variables hold -1 or
' the type's largest value. Result types are read from the C++ (qb64pe -z).
ON ERROR GOTO h
DIM sb AS _BYTE, ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM so AS _OFFSET, uo AS _UNSIGNED _OFFSET, b3 AS _BIT * 3, u3 AS _UNSIGNED _BIT * 3
' A _BIT * 40 writes 8 bytes into 4 and overwrites the _BIT scalar allocated before it (D-009,
' v21_b_bit_overlap): pad takes the overflow.
DIM pad AS _BIT * 32, b40 AS _BIT * 40
sb = -128: ub = 255: ui = 65535: ul = 4294967295: uq = 18446744073709551615~&&
so = -1: uo = 18446744073709551615~&&: b3 = -4: u3 = 7
PRINT "STR$: ["; STR$(sb); "]["; STR$(ub); "]["; STR$(ui); "]["; STR$(ul); "]["; STR$(uq); "]["; STR$(so); "]["; STR$(uo); "]["; STR$(b3); "]["; STR$(u3); "]"
PRINT "HEX$: "; HEX$(sb); " "; HEX$(ub); " "; HEX$(ui); " "; HEX$(ul); " "; HEX$(uq); " "; HEX$(so); " "; HEX$(uo); " "; HEX$(b3); " "; HEX$(u3)
PRINT "OCT$: "; OCT$(sb); " "; OCT$(ub); " "; OCT$(ui); " "; OCT$(ul); " "; OCT$(uq); " "; OCT$(so); " "; OCT$(b3); " "; OCT$(u3)
PRINT "_BIN$: "; _BIN$(sb); " "; _BIN$(ub); " "; _BIN$(b3); " "; _BIN$(u3); " "; _BIN$(so)
PRINT "HEX$ of -2 + 0 forms: "; HEX$(sb + 0); " "; HEX$(ub + 0); " "; HEX$(ul + 0); " "; HEX$(uq + 0); " "; HEX$(b3 + 0)
PRINT "ABS:"; ABS(sb); ABS(ub); ABS(ui); ABS(ul); ABS(uq); ABS(so); ABS(uo); ABS(b3); ABS(u3)
PRINT "SGN:"; SGN(sb); SGN(ub); SGN(ui); SGN(ul); SGN(uq); SGN(so); SGN(uo); SGN(b3); SGN(u3)
PRINT "INT:"; INT(sb); INT(ub); INT(ul); INT(uq); INT(so); INT(b3)
PRINT "FIX:"; FIX(sb); FIX(ub); FIX(ul); FIX(uq); FIX(so); FIX(b3)
PRINT "INT + 0:"; INT(ub) + 0; INT(uq) + 0
PRINT "CINT:"; CINT(sb); CINT(ub); CINT(b3)
PRINT "CINT ~%:"; CINT(ui)
PRINT "CLNG:"; CLNG(ui); CLNG(b3)
PRINT "CLNG ~&:"; CLNG(ul)
PRINT "CLNG ~&&:"; CLNG(uq)
PRINT "_ROUND:"; _ROUND(ub); _ROUND(ul); _ROUND(uq); _ROUND(uo); _ROUND(b3)
PRINT "CSNG:"; CSNG(ul); CSNG(uq); CSNG(b3)
PRINT "CDBL:"; CDBL(ul); CDBL(uq); CDBL(b3)
ub = 2: ui = 2: ul = 2: uq = 2: so = 2: uo = 2: sb = 2: b3 = 2: u3 = 2: b40 = 2
PRINT "SQR %% ~%% ~%:"; SQR(sb); SQR(ub); SQR(ui)
PRINT "SQR ~& ~&&:"; SQR(ul); SQR(uq)
PRINT "SQR %& ~%&:"; SQR(so); SQR(uo)
PRINT "SQR `3 ~`3 `40:"; SQR(b3); SQR(u3); SQR(b40)
PRINT "EXP %% ~%% ~%:"; EXP(sb); EXP(ub); EXP(ui)
PRINT "EXP ~& ~&& %&:"; EXP(ul); EXP(uq); EXP(so)
PRINT "EXP `3 `40:"; EXP(b3); EXP(b40)
PRINT "SIN ~&&, `3:"; SIN(uq); SIN(b3)
PRINT "VAL _BYTE:"; VAL("300", _BYTE); VAL("-1", _BYTE)
PRINT "VAL _UNSIGNED _BYTE:"; VAL("300", _UNSIGNED _BYTE); VAL("-1", _UNSIGNED _BYTE)
PRINT "VAL _UNSIGNED INTEGER:"; VAL("-1", _UNSIGNED INTEGER)
PRINT "VAL _UNSIGNED LONG:"; VAL("-1", _UNSIGNED LONG); VAL("4294967296", _UNSIGNED LONG)
PRINT "VAL _UNSIGNED _INTEGER64:"; VAL("-1", _UNSIGNED _INTEGER64); VAL("18446744073709551615", _UNSIGNED _INTEGER64)
PRINT "VAL _OFFSET:"; VAL("-1", _OFFSET); VAL("-1", _UNSIGNED _OFFSET)
PRINT "VAL _UNSIGNED _BYTE + 0:"; VAL("300", _UNSIGNED _BYTE) + 0
uq = 4294967298
ub = 66
b3 = 2
PRINT "LONG slot: ["; LEFT$("abcdef", uq); "] ["; CHR$(ub); "] ["; LEFT$("abcdef", b3); "]"
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
