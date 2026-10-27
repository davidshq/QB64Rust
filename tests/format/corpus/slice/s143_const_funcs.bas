$Console:Only
' The functions of the CONST evaluator (m3-constant-evaluation; verification\v45_b_funcs, v45_b_funcs_plain,
' v45_b_colors32, v45_b_colors_m*): each function as a _FLOAT constant, printed with _TOSTR$'s 19 digits, some
' as plain constants, the 32-bit colour functions and the palette colour functions of every screen mode. Not
' here: COS(1000), SIN(100) and 3 ^ 41, whose values are QB64PE-DIVERGENCES.md D-026's (crates\driver\tests\consts_v45.rs),
' and _COTH(.2) and 2 ^ -1, fixed by D-031 and D-028 (const-rulings; the same test, and the CLI test
' const_rulings_run).
' Built by the old compiler on another machine, the last digits of the others may differ too (D-026).
Const f01## = _Pi
Const f02## = _Pi(2)
Const f03## = _Pi(0.5)
Const f04## = _Acos(.2)
Const f05## = _Acos(-.5)
Const f06## = _Asin(.2)
Const f07## = _Asin(1)
Const f08## = _Arcsec(1.2)
Const f09## = _Arcsec(-3)
Const f10## = _Arccot(.2)
Const f11## = _Arccot(-2)
Const f12## = _Arccsc(1.2)
Const f13## = _Sech(.2)
Const f14## = _Sech(-3)
Const f15## = _Csch(.2)
Const f17## = Cos(.2)
Const f18## = Cos(1)
Const f19## = Cos(10)
Const f20## = Sin(.2)
Const f21## = Sin(1)
Const f22## = Sin(3)
Const f23## = Tan(.2)
Const f24## = Tan(1.5)
Const f25## = Log(.2)
Const f26## = Log(10)
Const f27## = Log(1E+100)
Const f28## = Exp(.2)
Const f29## = Exp(1)
Const f30## = Exp(-5)
Const f31## = Exp(20)
Const f32## = Atn(.2)
Const f33## = Atn(1)
Const f34## = Atn(1E+10)
Const f35## = _D2R(.2)
Const f36## = _D2R(180)
Const f37## = _D2G(.2)
Const f38## = _R2D(.2)
Const f39## = _R2G(.2)
Const f40## = _G2D(.2)
Const f41## = _G2R(.2)
Const f42## = Abs(2)
Const f43## = Abs(-2.5)
Const f44## = Sgn(-20)
Const f45## = Sgn(0)
Const f46## = Sgn(.1)
Const f47## = Int(20.2)
Const f48## = Int(-20.2)
Const f49## = _Round(20.5)
Const f50## = _Round(21.5)
Const f51## = _Round(-2.5)
Const f52## = _Ceil(20.2)
Const f53## = _Ceil(-20.2)
Const f54## = Fix(20.5)
Const f55## = Fix(-20.5)
Const f56## = _Sec(2)
Const f57## = _Csc(2)
Const f58## = _Cot(2)
Const f59## = Sqr(20)
Const f60## = Sqr(2)
Const f61## = Sqr(1E+300)
Const f62## = 20 ROOT 3
Const f63## = 8 ROOT 3
Const f64## = 2 ROOT 0.5
Const f65## = -8 ROOT 3
Const f66## = 2 ^ 0.3
Const f67## = 10 ^ 0.5
Const f68## = 1.5 ^ 2
Const f70## = 2.5 ^ 3
Const f71## = Sin(1##)
Const f72## = Exp(1.0##)
Print "_PI: "; _ToStr$(f01##)
Print "_PI(2): "; _ToStr$(f02##)
Print "_PI(0.5): "; _ToStr$(f03##)
Print "_ACOS(.2): "; _ToStr$(f04##)
Print "_ACOS(-.5): "; _ToStr$(f05##)
Print "_ASIN(.2): "; _ToStr$(f06##)
Print "_ASIN(1): "; _ToStr$(f07##)
Print "_ARCSEC(1.2): "; _ToStr$(f08##)
Print "_ARCSEC(-3): "; _ToStr$(f09##)
Print "_ARCCOT(.2): "; _ToStr$(f10##)
Print "_ARCCOT(-2): "; _ToStr$(f11##)
Print "_ARCCSC(1.2): "; _ToStr$(f12##)
Print "_SECH(.2): "; _ToStr$(f13##)
Print "_SECH(-3): "; _ToStr$(f14##)
Print "_CSCH(.2): "; _ToStr$(f15##)
Print "COS(.2): "; _ToStr$(f17##)
Print "COS(1): "; _ToStr$(f18##)
Print "COS(10): "; _ToStr$(f19##)
Print "SIN(.2): "; _ToStr$(f20##)
Print "SIN(1): "; _ToStr$(f21##)
Print "SIN(3): "; _ToStr$(f22##)
Print "TAN(.2): "; _ToStr$(f23##)
Print "TAN(1.5): "; _ToStr$(f24##)
Print "LOG(.2): "; _ToStr$(f25##)
Print "LOG(10): "; _ToStr$(f26##)
Print "LOG(1E+100): "; _ToStr$(f27##)
Print "EXP(.2): "; _ToStr$(f28##)
Print "EXP(1): "; _ToStr$(f29##)
Print "EXP(-5): "; _ToStr$(f30##)
Print "EXP(20): "; _ToStr$(f31##)
Print "ATN(.2): "; _ToStr$(f32##)
Print "ATN(1): "; _ToStr$(f33##)
Print "ATN(1E+10): "; _ToStr$(f34##)
Print "_D2R(.2): "; _ToStr$(f35##)
Print "_D2R(180): "; _ToStr$(f36##)
Print "_D2G(.2): "; _ToStr$(f37##)
Print "_R2D(.2): "; _ToStr$(f38##)
Print "_R2G(.2): "; _ToStr$(f39##)
Print "_G2D(.2): "; _ToStr$(f40##)
Print "_G2R(.2): "; _ToStr$(f41##)
Print "ABS(2): "; _ToStr$(f42##)
Print "ABS(-2.5): "; _ToStr$(f43##)
Print "SGN(-20): "; _ToStr$(f44##)
Print "SGN(0): "; _ToStr$(f45##)
Print "SGN(.1): "; _ToStr$(f46##)
Print "INT(20.2): "; _ToStr$(f47##)
Print "INT(-20.2): "; _ToStr$(f48##)
Print "_ROUND(20.5): "; _ToStr$(f49##)
Print "_ROUND(21.5): "; _ToStr$(f50##)
Print "_ROUND(-2.5): "; _ToStr$(f51##)
Print "_CEIL(20.2): "; _ToStr$(f52##)
Print "_CEIL(-20.2): "; _ToStr$(f53##)
Print "FIX(20.5): "; _ToStr$(f54##)
Print "FIX(-20.5): "; _ToStr$(f55##)
Print "_SEC(2): "; _ToStr$(f56##)
Print "_CSC(2): "; _ToStr$(f57##)
Print "_COT(2): "; _ToStr$(f58##)
Print "SQR(20): "; _ToStr$(f59##)
Print "SQR(2): "; _ToStr$(f60##)
Print "SQR(1E+300): "; _ToStr$(f61##)
Print "20 ROOT 3: "; _ToStr$(f62##)
Print "8 ROOT 3: "; _ToStr$(f63##)
Print "2 ROOT 0.5: "; _ToStr$(f64##)
Print "-8 ROOT 3: "; _ToStr$(f65##)
Print "2 ^ 0.3: "; _ToStr$(f66##)
Print "10 ^ 0.5: "; _ToStr$(f67##)
Print "1.5 ^ 2: "; _ToStr$(f68##)
Print "2.5 ^ 3: "; _ToStr$(f70##)
Print "SIN(1##): "; _ToStr$(f71##)
Print "EXP(1.0##): "; _ToStr$(f72##)
Const d1 = Sin(.2)
Print "SIN(.2): "; _ToStr$(d1); d1
Const d2 = Log(10)
Print "LOG(10): "; _ToStr$(d2); d2
Const d3 = Exp(1)
Print "EXP(1): "; _ToStr$(d3); d3
Const d4 = 20 ROOT 3
Print "20 ROOT 3: "; _ToStr$(d4); d4
Const d5 = Sqr(2)
Print "SQR(2): "; _ToStr$(d5); d5
Const d6 = _Pi(3)
Print "_PI(3): "; _ToStr$(d6); d6
Const d7 = _Acos(.2)
Print "_ACOS(.2): "; _ToStr$(d7); d7
Const d8 = Atn(1E+10)
Print "ATN(1E+10): "; _ToStr$(d8); d8
Const c1 = _RGB32(2, 3, 4)
Print "_RGB32(2, 3, 4):"; Hex$(c1); c1
Const c2 = _RGB32(2)
Print "_RGB32(2):"; Hex$(c2); c2
Const c3 = _RGB32(2, 3)
Print "_RGB32(2, 3):"; Hex$(c3); c3
Const c4 = _RGB32(2, 3, 4, 5)
Print "_RGB32(2, 3, 4, 5):"; Hex$(c4); c4
Const c5 = _RGB32(300, -1, 2.6)
Print "_RGB32(300, -1, 2.6):"; Hex$(c5); c5
Const c6 = _RGBA32(2, 3, 4, 5)
Print "_RGBA32(2, 3, 4, 5):"; Hex$(c6); c6
Const c7 = _RGBA32(256, 3, 4, 5)
Print "_RGBA32(256, 3, 4, 5):"; Hex$(c7); c7
Const c8 = _Red32(&H80112233)
Print "_RED32(&H80112233):"; Hex$(c8); c8
Const c9 = _Green32(&H80112233)
Print "_GREEN32(&H80112233):"; Hex$(c9); c9
Const c10 = _Blue32(&H80112233)
Print "_BLUE32(&H80112233):"; Hex$(c10); c10
Const c11 = _Alpha32(&H80112233)
Print "_ALPHA32(&H80112233):"; Hex$(c11); c11
Const c12 = _Red32(22)
Print "_RED32(22):"; Hex$(c12); c12
Const c13 = _Alpha32(-1)
Print "_ALPHA32(-1):"; Hex$(c13); c13
Const c14 = _RGB(255, 0, 0, 0)
Print "_RGB(255, 0, 0, 0):"; Hex$(c14)
Const c15 = _RGB(100, 150, 200, 0)
Print "_RGB(100, 150, 200, 0):"; Hex$(c15)
Const c16 = _RGB(2, 3, 4, 0)
Print "_RGB(2, 3, 4, 0):"; Hex$(c16)
Const c17 = _RGB(255, 255, 255, 0)
Print "_RGB(255, 255, 255, 0):"; Hex$(c17)
Const c18 = _RGB(0, 0, 170, 0)
Print "_RGB(0, 0, 170, 0):"; Hex$(c18)
Const c19 = _RGB(250, 80, 80, 0)
Print "_RGB(250, 80, 80, 0):"; Hex$(c19)
Const c20 = _RGBA(100, 150, 200, 128, 0)
Print "_RGBA(100, 150, 200, 128, 0):"; Hex$(c20)
Const c21 = _Red(1, 0)
Print "_RED(1, 0):"; c21
Const c22 = _Green(1, 0)
Print "_GREEN(1, 0):"; c22
Const c23 = _Blue(1, 0)
Print "_BLUE(1, 0):"; c23
Const c24 = _Alpha(1, 0)
Print "_ALPHA(1, 0):"; c24
Const c25 = _Red(5, 0)
Print "_RED(5, 0):"; c25
Const c26 = _Green(5, 0)
Print "_GREEN(5, 0):"; c26
Const c27 = _Blue(5, 0)
Print "_BLUE(5, 0):"; c27
Const c28 = _Alpha(5, 0)
Print "_ALPHA(5, 0):"; c28
Const c29 = _Red(15, 0)
Print "_RED(15, 0):"; c29
Const c30 = _Green(15, 0)
Print "_GREEN(15, 0):"; c30
Const c31 = _Blue(15, 0)
Print "_BLUE(15, 0):"; c31
Const c32 = _Alpha(15, 0)
Print "_ALPHA(15, 0):"; c32
Const c33 = _RGB(255, 0, 0, 1)
Print "_RGB(255, 0, 0, 1):"; Hex$(c33)
Const c34 = _RGB(100, 150, 200, 1)
Print "_RGB(100, 150, 200, 1):"; Hex$(c34)
Const c35 = _RGB(2, 3, 4, 1)
Print "_RGB(2, 3, 4, 1):"; Hex$(c35)
Const c36 = _RGB(255, 255, 255, 1)
Print "_RGB(255, 255, 255, 1):"; Hex$(c36)
Const c37 = _RGB(0, 0, 170, 1)
Print "_RGB(0, 0, 170, 1):"; Hex$(c37)
Const c38 = _RGB(250, 80, 80, 1)
Print "_RGB(250, 80, 80, 1):"; Hex$(c38)
Const c39 = _RGBA(100, 150, 200, 128, 1)
Print "_RGBA(100, 150, 200, 128, 1):"; Hex$(c39)
Const c40 = _Red(1, 1)
Print "_RED(1, 1):"; c40
Const c41 = _Green(1, 1)
Print "_GREEN(1, 1):"; c41
Const c42 = _Blue(1, 1)
Print "_BLUE(1, 1):"; c42
Const c43 = _Alpha(1, 1)
Print "_ALPHA(1, 1):"; c43
Const c44 = _Red(3, 1)
Print "_RED(3, 1):"; c44
Const c45 = _Green(3, 1)
Print "_GREEN(3, 1):"; c45
Const c46 = _Blue(3, 1)
Print "_BLUE(3, 1):"; c46
Const c47 = _Alpha(3, 1)
Print "_ALPHA(3, 1):"; c47
Const c48 = _RGB(255, 0, 0, 2)
Print "_RGB(255, 0, 0, 2):"; Hex$(c48)
Const c49 = _RGB(100, 150, 200, 2)
Print "_RGB(100, 150, 200, 2):"; Hex$(c49)
Const c50 = _RGB(2, 3, 4, 2)
Print "_RGB(2, 3, 4, 2):"; Hex$(c50)
Const c51 = _RGB(255, 255, 255, 2)
Print "_RGB(255, 255, 255, 2):"; Hex$(c51)
Const c52 = _RGB(0, 0, 170, 2)
Print "_RGB(0, 0, 170, 2):"; Hex$(c52)
Const c53 = _RGB(250, 80, 80, 2)
Print "_RGB(250, 80, 80, 2):"; Hex$(c53)
Const c54 = _RGBA(100, 150, 200, 128, 2)
Print "_RGBA(100, 150, 200, 128, 2):"; Hex$(c54)
Const c55 = _Red(0, 2)
Print "_RED(0, 2):"; c55
Const c56 = _Green(0, 2)
Print "_GREEN(0, 2):"; c56
Const c57 = _Blue(0, 2)
Print "_BLUE(0, 2):"; c57
Const c58 = _Alpha(0, 2)
Print "_ALPHA(0, 2):"; c58
Const c59 = _Red(1, 2)
Print "_RED(1, 2):"; c59
Const c60 = _Green(1, 2)
Print "_GREEN(1, 2):"; c60
Const c61 = _Blue(1, 2)
Print "_BLUE(1, 2):"; c61
Const c62 = _Alpha(1, 2)
Print "_ALPHA(1, 2):"; c62
Const c63 = _RGB(255, 0, 0, 7)
Print "_RGB(255, 0, 0, 7):"; Hex$(c63)
Const c64 = _RGB(100, 150, 200, 7)
Print "_RGB(100, 150, 200, 7):"; Hex$(c64)
Const c65 = _RGB(2, 3, 4, 7)
Print "_RGB(2, 3, 4, 7):"; Hex$(c65)
Const c66 = _RGB(255, 255, 255, 7)
Print "_RGB(255, 255, 255, 7):"; Hex$(c66)
Const c67 = _RGB(0, 0, 170, 7)
Print "_RGB(0, 0, 170, 7):"; Hex$(c67)
Const c68 = _RGB(250, 80, 80, 7)
Print "_RGB(250, 80, 80, 7):"; Hex$(c68)
Const c69 = _RGBA(100, 150, 200, 128, 7)
Print "_RGBA(100, 150, 200, 128, 7):"; Hex$(c69)
Const c70 = _Red(1, 7)
Print "_RED(1, 7):"; c70
Const c71 = _Green(1, 7)
Print "_GREEN(1, 7):"; c71
Const c72 = _Blue(1, 7)
Print "_BLUE(1, 7):"; c72
Const c73 = _Alpha(1, 7)
Print "_ALPHA(1, 7):"; c73
Const c74 = _Red(5, 7)
Print "_RED(5, 7):"; c74
Const c75 = _Green(5, 7)
Print "_GREEN(5, 7):"; c75
Const c76 = _Blue(5, 7)
Print "_BLUE(5, 7):"; c76
Const c77 = _Alpha(5, 7)
Print "_ALPHA(5, 7):"; c77
Const c78 = _Red(15, 7)
Print "_RED(15, 7):"; c78
Const c79 = _Green(15, 7)
Print "_GREEN(15, 7):"; c79
Const c80 = _Blue(15, 7)
Print "_BLUE(15, 7):"; c80
Const c81 = _Alpha(15, 7)
Print "_ALPHA(15, 7):"; c81
Const c82 = _RGB(255, 0, 0, 8)
Print "_RGB(255, 0, 0, 8):"; Hex$(c82)
Const c83 = _RGB(100, 150, 200, 8)
Print "_RGB(100, 150, 200, 8):"; Hex$(c83)
Const c84 = _RGB(2, 3, 4, 8)
Print "_RGB(2, 3, 4, 8):"; Hex$(c84)
Const c85 = _RGB(255, 255, 255, 8)
Print "_RGB(255, 255, 255, 8):"; Hex$(c85)
Const c86 = _RGB(0, 0, 170, 8)
Print "_RGB(0, 0, 170, 8):"; Hex$(c86)
Const c87 = _RGB(250, 80, 80, 8)
Print "_RGB(250, 80, 80, 8):"; Hex$(c87)
Const c88 = _RGBA(100, 150, 200, 128, 8)
Print "_RGBA(100, 150, 200, 128, 8):"; Hex$(c88)
Const c89 = _Red(1, 8)
Print "_RED(1, 8):"; c89
Const c90 = _Green(1, 8)
Print "_GREEN(1, 8):"; c90
Const c91 = _Blue(1, 8)
Print "_BLUE(1, 8):"; c91
Const c92 = _Alpha(1, 8)
Print "_ALPHA(1, 8):"; c92
Const c93 = _Red(5, 8)
Print "_RED(5, 8):"; c93
Const c94 = _Green(5, 8)
Print "_GREEN(5, 8):"; c94
Const c95 = _Blue(5, 8)
Print "_BLUE(5, 8):"; c95
Const c96 = _Alpha(5, 8)
Print "_ALPHA(5, 8):"; c96
Const c97 = _Red(15, 8)
Print "_RED(15, 8):"; c97
Const c98 = _Green(15, 8)
Print "_GREEN(15, 8):"; c98
Const c99 = _Blue(15, 8)
Print "_BLUE(15, 8):"; c99
Const c100 = _Alpha(15, 8)
Print "_ALPHA(15, 8):"; c100
Const c101 = _RGB(255, 0, 0, 9)
Print "_RGB(255, 0, 0, 9):"; Hex$(c101)
Const c102 = _RGB(100, 150, 200, 9)
Print "_RGB(100, 150, 200, 9):"; Hex$(c102)
Const c103 = _RGB(2, 3, 4, 9)
Print "_RGB(2, 3, 4, 9):"; Hex$(c103)
Const c104 = _RGB(255, 255, 255, 9)
Print "_RGB(255, 255, 255, 9):"; Hex$(c104)
Const c105 = _RGB(0, 0, 170, 9)
Print "_RGB(0, 0, 170, 9):"; Hex$(c105)
Const c106 = _RGB(250, 80, 80, 9)
Print "_RGB(250, 80, 80, 9):"; Hex$(c106)
Const c107 = _RGBA(100, 150, 200, 128, 9)
Print "_RGBA(100, 150, 200, 128, 9):"; Hex$(c107)
Const c108 = _Red(1, 9)
Print "_RED(1, 9):"; c108
Const c109 = _Green(1, 9)
Print "_GREEN(1, 9):"; c109
Const c110 = _Blue(1, 9)
Print "_BLUE(1, 9):"; c110
Const c111 = _Alpha(1, 9)
Print "_ALPHA(1, 9):"; c111
Const c112 = _Red(5, 9)
Print "_RED(5, 9):"; c112
Const c113 = _Green(5, 9)
Print "_GREEN(5, 9):"; c113
Const c114 = _Blue(5, 9)
Print "_BLUE(5, 9):"; c114
Const c115 = _Alpha(5, 9)
Print "_ALPHA(5, 9):"; c115
Const c116 = _Red(15, 9)
Print "_RED(15, 9):"; c116
Const c117 = _Green(15, 9)
Print "_GREEN(15, 9):"; c117
Const c118 = _Blue(15, 9)
Print "_BLUE(15, 9):"; c118
Const c119 = _Alpha(15, 9)
Print "_ALPHA(15, 9):"; c119
Const c120 = _RGB(255, 0, 0, 10)
Print "_RGB(255, 0, 0, 10):"; Hex$(c120)
Const c121 = _RGB(100, 150, 200, 10)
Print "_RGB(100, 150, 200, 10):"; Hex$(c121)
Const c122 = _RGB(2, 3, 4, 10)
Print "_RGB(2, 3, 4, 10):"; Hex$(c122)
Const c123 = _RGB(255, 255, 255, 10)
Print "_RGB(255, 255, 255, 10):"; Hex$(c123)
Const c124 = _RGB(0, 0, 170, 10)
Print "_RGB(0, 0, 170, 10):"; Hex$(c124)
Const c125 = _RGB(250, 80, 80, 10)
Print "_RGB(250, 80, 80, 10):"; Hex$(c125)
Const c126 = _RGBA(100, 150, 200, 128, 10)
Print "_RGBA(100, 150, 200, 128, 10):"; Hex$(c126)
Const c127 = _Red(1, 10)
Print "_RED(1, 10):"; c127
Const c128 = _Green(1, 10)
Print "_GREEN(1, 10):"; c128
Const c129 = _Blue(1, 10)
Print "_BLUE(1, 10):"; c129
Const c130 = _Alpha(1, 10)
Print "_ALPHA(1, 10):"; c130
Const c131 = _Red(3, 10)
Print "_RED(3, 10):"; c131
Const c132 = _Green(3, 10)
Print "_GREEN(3, 10):"; c132
Const c133 = _Blue(3, 10)
Print "_BLUE(3, 10):"; c133
Const c134 = _Alpha(3, 10)
Print "_ALPHA(3, 10):"; c134
Const c135 = _RGB(255, 0, 0, 11)
Print "_RGB(255, 0, 0, 11):"; Hex$(c135)
Const c136 = _RGB(100, 150, 200, 11)
Print "_RGB(100, 150, 200, 11):"; Hex$(c136)
Const c137 = _RGB(2, 3, 4, 11)
Print "_RGB(2, 3, 4, 11):"; Hex$(c137)
Const c138 = _RGB(255, 255, 255, 11)
Print "_RGB(255, 255, 255, 11):"; Hex$(c138)
Const c139 = _RGB(0, 0, 170, 11)
Print "_RGB(0, 0, 170, 11):"; Hex$(c139)
Const c140 = _RGB(250, 80, 80, 11)
Print "_RGB(250, 80, 80, 11):"; Hex$(c140)
Const c141 = _RGBA(100, 150, 200, 128, 11)
Print "_RGBA(100, 150, 200, 128, 11):"; Hex$(c141)
Const c142 = _Red(0, 11)
Print "_RED(0, 11):"; c142
Const c143 = _Green(0, 11)
Print "_GREEN(0, 11):"; c143
Const c144 = _Blue(0, 11)
Print "_BLUE(0, 11):"; c144
Const c145 = _Alpha(0, 11)
Print "_ALPHA(0, 11):"; c145
Const c146 = _Red(1, 11)
Print "_RED(1, 11):"; c146
Const c147 = _Green(1, 11)
Print "_GREEN(1, 11):"; c147
Const c148 = _Blue(1, 11)
Print "_BLUE(1, 11):"; c148
Const c149 = _Alpha(1, 11)
Print "_ALPHA(1, 11):"; c149
Const c150 = _RGB(255, 0, 0, 12)
Print "_RGB(255, 0, 0, 12):"; Hex$(c150)
Const c151 = _RGB(100, 150, 200, 12)
Print "_RGB(100, 150, 200, 12):"; Hex$(c151)
Const c152 = _RGB(2, 3, 4, 12)
Print "_RGB(2, 3, 4, 12):"; Hex$(c152)
Const c153 = _RGB(255, 255, 255, 12)
Print "_RGB(255, 255, 255, 12):"; Hex$(c153)
Const c154 = _RGB(0, 0, 170, 12)
Print "_RGB(0, 0, 170, 12):"; Hex$(c154)
Const c155 = _RGB(250, 80, 80, 12)
Print "_RGB(250, 80, 80, 12):"; Hex$(c155)
Const c156 = _RGBA(100, 150, 200, 128, 12)
Print "_RGBA(100, 150, 200, 128, 12):"; Hex$(c156)
Const c157 = _Red(1, 12)
Print "_RED(1, 12):"; c157
Const c158 = _Green(1, 12)
Print "_GREEN(1, 12):"; c158
Const c159 = _Blue(1, 12)
Print "_BLUE(1, 12):"; c159
Const c160 = _Alpha(1, 12)
Print "_ALPHA(1, 12):"; c160
Const c161 = _Red(5, 12)
Print "_RED(5, 12):"; c161
Const c162 = _Green(5, 12)
Print "_GREEN(5, 12):"; c162
Const c163 = _Blue(5, 12)
Print "_BLUE(5, 12):"; c163
Const c164 = _Alpha(5, 12)
Print "_ALPHA(5, 12):"; c164
Const c165 = _Red(15, 12)
Print "_RED(15, 12):"; c165
Const c166 = _Green(15, 12)
Print "_GREEN(15, 12):"; c166
Const c167 = _Blue(15, 12)
Print "_BLUE(15, 12):"; c167
Const c168 = _Alpha(15, 12)
Print "_ALPHA(15, 12):"; c168
Const c169 = _RGB(255, 0, 0, 13)
Print "_RGB(255, 0, 0, 13):"; Hex$(c169)
Const c170 = _RGB(100, 150, 200, 13)
Print "_RGB(100, 150, 200, 13):"; Hex$(c170)
Const c171 = _RGB(2, 3, 4, 13)
Print "_RGB(2, 3, 4, 13):"; Hex$(c171)
Const c172 = _RGB(255, 255, 255, 13)
Print "_RGB(255, 255, 255, 13):"; Hex$(c172)
Const c173 = _RGB(0, 0, 170, 13)
Print "_RGB(0, 0, 170, 13):"; Hex$(c173)
Const c174 = _RGB(250, 80, 80, 13)
Print "_RGB(250, 80, 80, 13):"; Hex$(c174)
Const c175 = _RGBA(100, 150, 200, 128, 13)
Print "_RGBA(100, 150, 200, 128, 13):"; Hex$(c175)
Const c176 = _Red(1, 13)
Print "_RED(1, 13):"; c176
Const c177 = _Green(1, 13)
Print "_GREEN(1, 13):"; c177
Const c178 = _Blue(1, 13)
Print "_BLUE(1, 13):"; c178
Const c179 = _Alpha(1, 13)
Print "_ALPHA(1, 13):"; c179
Const c180 = _Red(5, 13)
Print "_RED(5, 13):"; c180
Const c181 = _Green(5, 13)
Print "_GREEN(5, 13):"; c181
Const c182 = _Blue(5, 13)
Print "_BLUE(5, 13):"; c182
Const c183 = _Alpha(5, 13)
Print "_ALPHA(5, 13):"; c183
Const c184 = _Red(15, 13)
Print "_RED(15, 13):"; c184
Const c185 = _Green(15, 13)
Print "_GREEN(15, 13):"; c185
Const c186 = _Blue(15, 13)
Print "_BLUE(15, 13):"; c186
Const c187 = _Alpha(15, 13)
Print "_ALPHA(15, 13):"; c187
Const c188 = _RGB(255, 0, 0, 256)
Print "_RGB(255, 0, 0, 256):"; Hex$(c188)
Const c189 = _RGB(100, 150, 200, 256)
Print "_RGB(100, 150, 200, 256):"; Hex$(c189)
Const c190 = _RGB(2, 3, 4, 256)
Print "_RGB(2, 3, 4, 256):"; Hex$(c190)
Const c191 = _RGB(255, 255, 255, 256)
Print "_RGB(255, 255, 255, 256):"; Hex$(c191)
Const c192 = _RGB(0, 0, 170, 256)
Print "_RGB(0, 0, 170, 256):"; Hex$(c192)
Const c193 = _RGB(250, 80, 80, 256)
Print "_RGB(250, 80, 80, 256):"; Hex$(c193)
Const c194 = _RGBA(100, 150, 200, 128, 256)
Print "_RGBA(100, 150, 200, 128, 256):"; Hex$(c194)
Const c195 = _Red(1, 256)
Print "_RED(1, 256):"; c195
Const c196 = _Green(1, 256)
Print "_GREEN(1, 256):"; c196
Const c197 = _Blue(1, 256)
Print "_BLUE(1, 256):"; c197
Const c198 = _Alpha(1, 256)
Print "_ALPHA(1, 256):"; c198
Const c199 = _Red(5, 256)
Print "_RED(5, 256):"; c199
Const c200 = _Green(5, 256)
Print "_GREEN(5, 256):"; c200
Const c201 = _Blue(5, 256)
Print "_BLUE(5, 256):"; c201
Const c202 = _Alpha(5, 256)
Print "_ALPHA(5, 256):"; c202
Const c203 = _Red(15, 256)
Print "_RED(15, 256):"; c203
Const c204 = _Green(15, 256)
Print "_GREEN(15, 256):"; c204
Const c205 = _Blue(15, 256)
Print "_BLUE(15, 256):"; c205
Const c206 = _Alpha(15, 256)
Print "_ALPHA(15, 256):"; c206
Const c207 = _RGB(255, 0, 0, 32)
Print "_RGB(255, 0, 0, 32):"; Hex$(c207)
Const c208 = _RGB(100, 150, 200, 32)
Print "_RGB(100, 150, 200, 32):"; Hex$(c208)
Const c209 = _RGB(2, 3, 4, 32)
Print "_RGB(2, 3, 4, 32):"; Hex$(c209)
Const c210 = _RGB(255, 255, 255, 32)
Print "_RGB(255, 255, 255, 32):"; Hex$(c210)
Const c211 = _RGB(0, 0, 170, 32)
Print "_RGB(0, 0, 170, 32):"; Hex$(c211)
Const c212 = _RGB(250, 80, 80, 32)
Print "_RGB(250, 80, 80, 32):"; Hex$(c212)
Const c213 = _RGBA(100, 150, 200, 128, 32)
Print "_RGBA(100, 150, 200, 128, 32):"; Hex$(c213)
Const c214 = _Red(&HFF112233, 32)
Print "_RED(&HFF112233, 32):"; c214
Const c215 = _Green(&HFF112233, 32)
Print "_GREEN(&HFF112233, 32):"; c215
Const c216 = _Blue(&HFF112233, 32)
Print "_BLUE(&HFF112233, 32):"; c216
Const c217 = _Alpha(&HFF112233, 32)
Print "_ALPHA(&HFF112233, 32):"; c217
Const c218 = _Red(&H80445566, 32)
Print "_RED(&H80445566, 32):"; c218
Const c219 = _Green(&H80445566, 32)
Print "_GREEN(&H80445566, 32):"; c219
Const c220 = _Blue(&H80445566, 32)
Print "_BLUE(&H80445566, 32):"; c220
Const c221 = _Alpha(&H80445566, 32)
Print "_ALPHA(&H80445566, 32):"; c221
Const c222 = _Red(22, 0)
Print "_RED(22, 0):"; c222
Const c223 = _Alpha(2222, 0)
Print "_ALPHA(2222, 0):"; c223
System
