$Console:Only
' CHR$ in a CONST (m3-errorline-chr-const; verification\v43_c_chr_const): each kind of number, floats near a half
' (rounded half to even), a constant, an expression and parentheses as the code, CHR$ beside other strings, several
' on one line, letter case, in a SUB, the constant used with and without its $ suffix. Also the constants of the
' auto-included file made with CHR$.
Const a = Chr$(65)
Const b$ = Chr$(66)
Const c = Chr$(0)
Const d = Chr$(255)
Const e = Chr$(65.4)
Const f = Chr$(65.5)
Const g = Chr$(66.5)
Const h = Chr$(65.6)
Const i = Chr$(&H43)
Const j = Chr$(68&&)
Const k = Chr$(69~%%)
Const l = Chr$(7E1)
Const n = 71
Const m = Chr$(n)
Const o = Chr$(30 + 42)
Const p = Chr$((73))
Const q = "x" + Chr$(74) + Chr$(75) + "y"
Const t = Chr$(255.4)
Const u = Chr$(2 ^ 6)
Const v = Chr$(77!)
Const w = Chr$(78#)
Const w2 = Chr$(&B1001111)
Const w3 = Chr$(&O120)
Const w4 = Chr$(65 + 0.5)
Const w5 = Chr$(100 / 2)
Const w6 = Chr$(n * 2 - 60)
Const w7 = Chr$(81)
Const w8 = Chr$(82) + Chr$(83), w9 = Chr$(84)
Const w10 = Chr$(34) + "a" + Chr$(0) + Chr$(10) + Chr$(92) + Chr$(63) + Chr$(63) + Chr$(61)
Const w11 = Chr$(-0.4)
Const w12 = Chr$(0.5)
Const w13 = Chr$(1.5)
Const w14 = Chr$(85~&)
Const w15 = Chr$(86%)
Const w16 = Chr$(87%%)
Const w17 = Chr$(88&)
Const w18 = Chr$(-0.5)
Const w19 = Chr$(131 / 2)
Const w20 = Chr$(Not -66)
Const w21 = (Chr$(65)) + "b"

Print "1."; Asc(a); Asc(b$); Asc(c); Asc(d); Len(c)
Print "2. halves:"; Asc(e); Asc(f); Asc(g); Asc(h); Asc(t); Asc(w4); Asc(w11); Asc(w12); Asc(w13); Asc(w18); Asc(w19)
Print "3. kinds:"; Asc(i); Asc(j); Asc(k); Asc(l); Asc(v); Asc(w); Asc(w2); Asc(w3); Asc(w14); Asc(w15); Asc(w16); Asc(w17)
Print "4. expressions:"; Asc(m); Asc(o); Asc(p); Asc(u); Asc(w5); Asc(w6); Asc(w7); Asc(w20)
Print "5. "; q; " "; w8; " "; w9; " "; a$; " "; a + b$; " "; Len(q); " "; w21
Print "6. bytes:";
For z = 1 To Len(w10): Print Asc(w10, z);: Next
Print
Print "7. auto-included:"; Len(_STR_CRLF); Asc(_STR_CRLF); Asc(_STR_CRLF, 2); Asc(_STR_LF); Asc(_CHR_NUL); Asc(_CHR_CR); Asc(_CHR_ESC); Len(_STR_NAT_EOL)
Const k2 = _CHR_CR + "x"
Print "8. "; Len(k2); Asc(k2)
insub
System

Sub insub
    Const s1 = Chr$(89) + a
    Print "9. in a SUB: "; s1
End Sub
