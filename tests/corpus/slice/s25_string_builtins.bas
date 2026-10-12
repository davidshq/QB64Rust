$CONSOLE:ONLY
' Slice program (m2-core-builtins, D9): the plain string built-ins and their edge values, measured in
' verification\v20_c_string_edges. Errors (ASC, CHR$, _TOSTR$) are trapped: the handler prints ERR and resumes next.
' No PRINT comma.
ON ERROR GOTO h
DIM s AS STRING, r AS STRING, n AS LONG
s = "abc"
PRINT LEFT$("hello", 2); LEN("abc")
PRINT "[" + LEFT$(s, 0) + "][" + LEFT$(s, 2) + "][" + LEFT$(s, 5) + "][" + LEFT$(s, -1) + "]"
PRINT "[" + RIGHT$(s, 0) + "][" + RIGHT$(s, 2) + "][" + RIGHT$(s, 5) + "][" + RIGHT$(s, -1) + "]"
PRINT MID$("hello", 2); MID$("hello", 2, 3)
PRINT "[" + MID$(s, 0) + "][" + MID$(s, 0, 2) + "][" + MID$(s, -1) + "][" + MID$(s, 3) + "][" + MID$(s, 4) + "][" + MID$(s, 5) + "]"
PRINT "[" + MID$(s, 2, 0) + "][" + MID$(s, 2, -1) + "][" + MID$(s, 2, 9) + "][" + MID$("", 1) + "]"
' Float arguments to LONG slots: rounded half to even; beyond LONG, the low 32 bits.
PRINT LEFT$("abcdef", 2.5); LEFT$("abcdef", 3.5); " "; LEFT$("abcdef", 4294967298#); " "; MID$("abcdef", 1.5, 2.5)
DIM big AS _INTEGER64
big = 4294967299
PRINT "["; LEFT$("abcdef", big); "]["; LEFT$("abcdef", 3E9); "]"
PRINT ASC("a"); ASC(s, 2); ASC(CHR$(200)); ASC("xyz", 3.4)
n = 7: n = ASC(""): PRINT "ASC empty:"; n
n = 7: n = ASC(s, 0): PRINT "ASC 0:"; n
n = 7: n = ASC(s, 4): PRINT "ASC past end:"; n
n = 7: n = ASC(s, -1): PRINT "ASC -1:"; n
r = "x": r = CHR$(256): PRINT "CHR$ 256: ["; r; "]"
PRINT CHR$(65.5); CHR$(66.5); CHR$(97)
PRINT "[" + STRING$(3, "xyz") + "][" + STRING$(3, 65) + "][" + STRING$(0, 65) + "][" + STRING$(-1, 65) + "]"
PRINT ASC(STRING$(2, 256)); ASC(STRING$(2, -1)); ASC(STRING$(1, 321)); LEN(STRING$(2, 256))
PRINT "[" + SPACE$(2) + "][" + SPACE$(0) + "][" + SPACE$(-1) + "]"
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
i = -5: l = 70000: q = 9007199254740993: f = 1 / 3: d = 1 / 3: x = 1 / 3
PRINT "[" + STR$(i) + "][" + STR$(l) + "][" + STR$(q) + "][" + STR$(f) + "][" + STR$(d) + "][" + STR$(x) + "]"
PRINT "[" + STR$(5) + "][" + STR$(-5) + "][" + STR$(-0) + "][" + STR$(2.5) + "][" + STR$(1D+300) + "][" + STR$(1.5E-07) + "]"
f = 1E+20
PRINT "[" + _TOSTR$(i) + "][" + _TOSTR$(f) + "][" + _TOSTR$(d) + "][" + _TOSTR$(1 / 3) + "]"
PRINT "[" + _TOSTR$(d, 3) + "][" + _TOSTR$(1 / 3, 5) + "][" + _TOSTR$(l, 2) + "][" + _TOSTR$(2.5, 0) + "]"
r = "x": r = _TOSTR$(2.5, -1): PRINT "_TOSTR$ -1: ["; r; "]"
r = CHR$(9) + " " + CHR$(0) + "a" + CHR$(0) + " " + CHR$(9)
PRINT LEN(LTRIM$(r)); LEN(RTRIM$(r)); LEN(_TRIM$(r))
r = "  a b  "
PRINT "[" + LTRIM$(r) + "][" + RTRIM$(r) + "][" + _TRIM$(r) + "][" + _TRIM$("") + "][" + LTRIM$("   ") + "]"
r = "aZ" + CHR$(130) + CHR$(154) + CHR$(228) + "~"
PRINT UCASE$("Hello, World 1"); " "; LCASE$("Hello, World 1")
PRINT ASC(UCASE$(r), 1); ASC(UCASE$(r), 2); ASC(UCASE$(r), 3); ASC(LCASE$(r), 2); ASC(LCASE$(r), 4); ASC(LCASE$(r), 6)
PRINT INSTR("abcabc", "c"); INSTR(4, "abcabc", "c"); INSTR("abc", ""); INSTR(0, "abc", "b"); INSTR(2.5, "abcabc", "b")
' Nested and in arithmetic.
PRINT LEN(LEFT$(s + s, 4) + MID$(s, 2)) * 2; ASC(RIGHT$(UCASE$(s), 1)) + 1; LEFT$(STR$(LEN(s)), 2)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
