$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): a float in the integer lanes (MOD, \, AND, OR, unary minus of an
' unsigned value), integer-valued floats near 2^63, the type by the text of small and large results
CONST i1 = 2.5 \ 1, i2 = 3.5 \ 1, i3 = -2.5 \ 1, i4 = 2.4999 \ 1, i5 = 7.5 MOD 4, i6 = -7.5 MOD 4
CONST u1 = 2.5 AND 255~&&, u2 = 3.5 OR 0~&&, u3 = 1E+19 OR 0~&&, u4 = -1.5 OR 0~&&
CONST s1 = 1E+19 OR 0, s2 = -1E+19 OR 0, s3 = 1E+30 OR 0
CONST n1 = -18446744073709551615~&&, n2 = NOT 0~&&, n3 = NOT 2.5
CONST g = 9.3E+18 / 1, g2 = 9.2E+18 / 1, g3 = 9.9E+18 / 1
CONST h = 1E+18 / 1, k = 1E-5 / 1, k2 = 0.0001 / 1, k3 = 123456789012345678 / 10
PRINT i1; i2; i3; i4; i5; i6
PRINT u1; u2; u3; u4
PRINT s1; s2; s3
PRINT n1; n2; n3
PRINT g; g2; g3
PRINT h; _TOSTR$(k); " "; _TOSTR$(k2); " "; _TOSTR$(k3)
SYSTEM
