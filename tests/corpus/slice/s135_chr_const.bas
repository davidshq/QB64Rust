$CONSOLE:ONLY
' CHR$ in a CONST (m3-errorline-chr-const; verification\v43_c_chr_const): each kind of number, floats near a half
' (rounded half to even), a constant, an expression and parentheses as the code, CHR$ beside other strings, several
' on one line, letter case, in a SUB, the constant used with and without its $ suffix. Also the constants of the
' auto-included file made with CHR$.
CONST a = CHR$(65)
CONST b$ = CHR$(66)
CONST c = CHR$(0)
CONST d = CHR$(255)
CONST e = CHR$(65.4)
CONST f = CHR$(65.5)
CONST g = CHR$(66.5)
CONST h = CHR$(65.6)
CONST i = CHR$(&H43)
CONST j = CHR$(68&&)
CONST k = CHR$(69~%%)
CONST l = CHR$(7E1)
CONST n = 71
CONST m = CHR$(n)
CONST o = CHR$(30 + 42)
CONST p = CHR$((73))
CONST q = "x" + CHR$(74) + CHR$(75) + "y"
CONST t = CHR$(255.4)
CONST u = CHR$(2 ^ 6)
CONST v = CHR$(77!)
CONST w = CHR$(78#)
CONST w2 = CHR$(&B1001111)
CONST w3 = CHR$(&O120)
CONST w4 = CHR$(65 + 0.5)
CONST w5 = CHR$(100 / 2)
CONST w6 = CHR$(n * 2 - 60)
CONST w7 = chr$(81)
CONST w8 = CHR$(82) + CHR$(83), w9 = CHR$(84)
CONST w10 = CHR$(34) + "a" + CHR$(0) + CHR$(10) + CHR$(92) + CHR$(63) + CHR$(63) + CHR$(61)
CONST w11 = CHR$(-0.4)
CONST w12 = CHR$(0.5)
CONST w13 = CHR$(1.5)
CONST w14 = CHR$(85~&)
CONST w15 = CHR$(86%)
CONST w16 = CHR$(87%%)
CONST w17 = CHR$(88&)
CONST w18 = CHR$(-0.5)
CONST w19 = CHR$(131 / 2)
CONST w20 = CHR$(NOT -66)
CONST w21 = (CHR$(65)) + "b"

PRINT "1."; ASC(a); ASC(b$); ASC(c); ASC(d); LEN(c)
PRINT "2. halves:"; ASC(e); ASC(f); ASC(g); ASC(h); ASC(t); ASC(w4); ASC(w11); ASC(w12); ASC(w13); ASC(w18); ASC(w19)
PRINT "3. kinds:"; ASC(i); ASC(j); ASC(k); ASC(l); ASC(v); ASC(w); ASC(w2); ASC(w3); ASC(w14); ASC(w15); ASC(w16); ASC(w17)
PRINT "4. expressions:"; ASC(m); ASC(o); ASC(p); ASC(u); ASC(w5); ASC(w6); ASC(w7); ASC(w20)
PRINT "5. "; q; " "; w8; " "; w9; " "; a$; " "; a + b$; " "; LEN(q); " "; w21
PRINT "6. bytes:";
FOR z = 1 TO LEN(w10): PRINT ASC(w10, z);: NEXT
PRINT
PRINT "7. auto-included:"; LEN(_STR_CRLF); ASC(_STR_CRLF); ASC(_STR_CRLF, 2); ASC(_STR_LF); ASC(_CHR_NUL); ASC(_CHR_CR); ASC(_CHR_ESC); LEN(_STR_NAT_EOL)
CONST k2 = _CHR_CR + "x"
PRINT "8. "; LEN(k2); ASC(k2)
insub
SYSTEM

SUB insub
    CONST s1 = CHR$(89) + a
    PRINT "9. in a SUB: "; s1
END SUB
