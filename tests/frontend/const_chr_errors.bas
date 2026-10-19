' TEST: check-fail
$CONSOLE:ONLY
' CHR$ in a CONST rejected (m3-errorline-chr-const; each form one program of verification\v43_x01-x28, where the
' old compiler rejects it too): a code outside 0 to 255, also after rounding half to even (256, -1, 255.5, -0.6,
' 1E300, the largest _UNSIGNED _INTEGER64), a string or a string constant as the code, CHR$ of CHR$, no code or two,
' no parentheses, an integer suffix on the name, the string mixed with a number, multiplied, compared, negated, and
' CHR without its dollar; 2^64 without a suffix, a literal beyond _UNSIGNED _INTEGER64 (DIVERGENCES.md D-027, an
' error of its own; the old compiler clamps it and rejects the code).
CONST c1 = CHR$(256)
CONST c2 = CHR$(-1)
CONST c3 = CHR$(255.5)
CONST c4 = CHR$(-0.6)
CONST c5 = CHR$(1E300)
CONST c6 = CHR$(18446744073709551615~&&)
CONST c7 = CHR$("A")
CONST s = "A"
CONST c8 = CHR$(s)
CONST c9 = CHR$(CHR$(65))
CONST c10 = CHR$()
CONST c11 = CHR$(1, 2)
CONST c12 = CHR$ 65
CONST c13% = CHR$(65)
CONST c14 = CHR$(65) + 1
CONST c15 = 1 + CHR$(65)
CONST c16 = CHR$(65) * 2
CONST c17 = CHR$(65) = "A"
CONST c18 = -CHR$(65)
CONST c19 = CHR(65)
x = 65
CONST c21 = CHR$(x)
CONST c20 = CHR$(18446744073709551616)
SYSTEM
