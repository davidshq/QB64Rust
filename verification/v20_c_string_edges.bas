$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): string built-ins at their edge values, and the errors they raise.
ON ERROR GOTO h
DIM s AS STRING, r AS STRING, n AS LONG
s = "abc"
r = "x": r = LEFT$(s, 0): PRINT "LEFT$ 0: ["; r; "]"
r = "x": r = LEFT$(s, 5): PRINT "LEFT$ 5: ["; r; "]"
r = "x": r = LEFT$(s, -1): PRINT "LEFT$ -1: ["; r; "]"
r = "x": r = RIGHT$(s, 0): PRINT "RIGHT$ 0: ["; r; "]"
r = "x": r = RIGHT$(s, 2): PRINT "RIGHT$ 2: ["; r; "]"
r = "x": r = RIGHT$(s, 5): PRINT "RIGHT$ 5: ["; r; "]"
r = "x": r = RIGHT$(s, -1): PRINT "RIGHT$ -1: ["; r; "]"
r = "x": r = MID$(s, 2): PRINT "MID$ 2: ["; r; "]"
r = "x": r = MID$(s, 2, 1): PRINT "MID$ 2 1: ["; r; "]"
r = "x": r = MID$(s, 0): PRINT "MID$ 0: ["; r; "]"
r = "x": r = MID$(s, 0, 2): PRINT "MID$ 0 2: ["; r; "]"
r = "x": r = MID$(s, -1): PRINT "MID$ -1: ["; r; "]"
r = "x": r = MID$(s, 3): PRINT "MID$ 3: ["; r; "]"
r = "x": r = MID$(s, 4): PRINT "MID$ 4: ["; r; "]"
r = "x": r = MID$(s, 5): PRINT "MID$ 5: ["; r; "]"
r = "x": r = MID$(s, 2, 0): PRINT "MID$ 2 0: ["; r; "]"
r = "x": r = MID$(s, 2, -1): PRINT "MID$ 2 -1: ["; r; "]"
r = "x": r = MID$(s, 2, 9): PRINT "MID$ 2 9: ["; r; "]"
r = "x": r = MID$("", 1): PRINT "MID$ empty 1: ["; r; "]"
n = 7: n = ASC(""): PRINT "ASC empty:"; n
n = 7: n = ASC(s): PRINT "ASC abc:"; n
n = 7: n = ASC(s, 2): PRINT "ASC abc 2:"; n
n = 7: n = ASC(s, 0): PRINT "ASC abc 0:"; n
n = 7: n = ASC(s, 4): PRINT "ASC abc 4:"; n
n = 7: n = ASC(s, -1): PRINT "ASC abc -1:"; n
n = 7: n = ASC("", 1): PRINT "ASC empty 1:"; n
n = 7: n = ASC(CHR$(200)): PRINT "ASC chr 200:"; n
r = "x": r = STRING$(3, "xyz"): PRINT "STRING$ 3 xyz: ["; r; "]"
r = "x": r = STRING$(3, 65): PRINT "STRING$ 3 65: ["; r; "]"
r = "x": r = STRING$(0, 65): PRINT "STRING$ 0 65: ["; r; "]"
r = "x": r = STRING$(-1, 65): PRINT "STRING$ -1 65: ["; r; "]"
r = "x": r = STRING$(2, 256): PRINT "STRING$ 2 256: ["; r; "]"
r = "x": r = STRING$(2, -1): PRINT "STRING$ 2 -1: ["; r; "]"
r = "x": r = STRING$(2, ""): PRINT "STRING$ 2 empty: ["; r; "]"
r = "x": r = SPACE$(2): PRINT "SPACE$ 2: ["; r; "]"
r = "x": r = SPACE$(0): PRINT "SPACE$ 0: ["; r; "]"
r = "x": r = SPACE$(-1): PRINT "SPACE$ -1: ["; r; "]"
r = "x": r = CHR$(256): PRINT "CHR$ 256: ["; r; "]"
r = "x": r = CHR$(-1): PRINT "CHR$ -1: ["; r; "]"
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
i = -5: l = 70000: q = 9007199254740993: f = 1 / 3: d = 1 / 3: x = 1 / 3
PRINT "STR$: ["; STR$(i); "]["; STR$(l); "]["; STR$(q); "]["; STR$(f); "]["; STR$(d); "]["; STR$(x); "]"
f = -0: d = 0: d = -d
PRINT "STR$ -0: ["; STR$(-0); "]["; STR$(f); "]["; STR$(d); "]"
f = 1E+20: d = 1D+300: x = 1E-20
PRINT "STR$ large small: ["; STR$(f); "]["; STR$(d); "]["; STR$(x); "]["; STR$(1.5E-7); "]"
PRINT "STR$ 0.1 + 0.2: ["; STR$(0.1 + 0.2); "]["; STR$(0.1# + 0.2#); "]"
PRINT "_TOSTR$: ["; _TOSTR$(i); "]["; _TOSTR$(f); "]["; _TOSTR$(d); "]["; _TOSTR$(1 / 3); "]"
PRINT "_TOSTR$ digits: ["; _TOSTR$(d, 3); "]["; _TOSTR$(1 / 3, 5); "]["; _TOSTR$(l, 2); "]["; _TOSTR$(2.5, 0); "]["; _TOSTR$(2.5, -1); "]"
r = CHR$(9) + " " + CHR$(0) + "a" + CHR$(0) + " " + CHR$(9)
PRINT "LTRIM$ len:"; LEN(LTRIM$(r)); " RTRIM$ len:"; LEN(RTRIM$(r)); " _TRIM$ len:"; LEN(_TRIM$(r))
r = "  a b  "
PRINT "LTRIM$: ["; LTRIM$(r); "] RTRIM$: ["; RTRIM$(r); "] _TRIM$: ["; _TRIM$(r); "]"
r = "aZ" + CHR$(130) + CHR$(154) + CHR$(228) + "~"
PRINT "UCASE$ codes:"; ASC(UCASE$(r), 1); ASC(UCASE$(r), 2); ASC(UCASE$(r), 3); ASC(UCASE$(r), 4); ASC(UCASE$(r), 5)
PRINT "LCASE$ codes:"; ASC(LCASE$(r), 1); ASC(LCASE$(r), 2); ASC(LCASE$(r), 3); ASC(LCASE$(r), 4); ASC(LCASE$(r), 5)
PRINT "INSTR: "; INSTR("abcabc", "c"); INSTR(4, "abcabc", "c"); INSTR("abc", ""); INSTR(0, "abc", "b"); INSTR(9, "abc", "b")
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
