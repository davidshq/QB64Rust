$Console:Only
_Dest _Console

' This list comprises all of the math functions usable in CONST
Const const_PI = _Pi
Const const_PIfunc = _Pi(2)
Const const_ACOS = _Acos(.2)
Const const_ASIN = _Asin(.2)
Const const_ARCSEC = _Arcsec(1.2)
Const const_ARCCOT = _Arccot(.2)
Const const_ARCCSC = _Arccsc(1.2)
Const const_SECH = _Sech(.2)
Const const_CSCH = _Csch(.2)
Const const_COTH = _Coth(.2)
Const const_COS = Cos(.2)
Const const_SIN = Sin(.2)
Const const_TAN = Tan(.2)
Const const_LOG = Log(.2)
Const const_EXP = Exp(.2)
Const const_ATN = Atn(.2)
Const const_D2R = _D2R(.2)
Const const_D2G = _D2G(.2)
Const const_R2D = _R2D(.2)
Const const_R2G = _R2G(.2)
Const const_G2D = _G2D(.2)
Const const_G2R = _G2R(.2)
Const const_ABS = Abs(2)
Const const_SGN = Sgn(-20)
Const const_INT = Int(20.2)
Const const_ROUND = _Round(20.2)
Const const_CEIL = _Ceil(20.2)
Const const_FIX = Fix(20.5)
Const const_SEC = _Sec(2)
Const const_CSC = _Csc(2)
Const const_COT = _Cot(2)
Const const__RGB32 = _RGB32(2, 3, 4)
Const const__RGBA32 = _RGBA32(2, 3, 4, 5)
Const const__RGB32_1 = _RGB32(2)
Const const__RGB32_2 = _RGB32(2, 3)
Const const__RGB32_4 = _RGB32(2, 3, 4, 5)
Const const__RGB = _RGB(2, 3, 4, 2)
Const const__RGBA = _RGBA(2, 3, 4, 2, 2)
Const const__RED32 = _Red32(22)
Const const__GREEN32 = _Green32(22)
Const const__BLUE32 = _Blue32(22)
Const const__ALPHA32 = _Alpha32(22)
Const const__RED = _Red(22, 0)
Const const__GREEN = _Green(22, 0)
Const const__BLUE = _Blue(22, 0)
Const const__ALPHA = _Alpha(2222, 0)
Const const_SQR = Sqr(20)
Const const_ROOT = 20 ROOT 3

Const const_CHR = Chr$(34) + Chr$(9)
Const const_ASC = Asc("\")
Const const_nested = Chr$(Asc(Chr$(Asc("\"))))

' The answers have to be within the allowed range, to account for floating point
' differences.
Print "PI: "; 3.141592653589793 * .999999 < const_PI; 3.141592653589793 * 1.000001 > const_PI
Print "PI: "; 6.283185307179586 * .999999 < const_PIfunc; 6.283185307179586 * 1.000001 > const_PIfunc
Print "ACOS: "; 1.369438406004566 * .999999 < const_ACOS; 1.369438406004566 * 1.000001 > const_ACOS
Print "ASIN: "; .2013579207903308 * .999999 < const_ASIN; .2013579207903308 * 1.000001 > const_ASIN
Print "ARCSEC: "; .5856855434571508 * .999999 < const_ARCSEC; .5856855434571508 * 1.000001 > const_ARCSEC
Print "ARCCOT: "; 1.373400766945016 * .999999 < const_ARCCOT; 1.373400766945016 * 1.000001 > const_ARCCOT
Print "ARCCSC: "; .9851107833377457 * .999999 < const_ARCCSC; .9851107833377457 * 1.000001 > const_ARCCSC
Print "SECH: "; .9803279976447253 * .999999 < const_SECH; .9803279976447253 * 1.000001 > const_SECH
Print "CSCH: "; 4.966821568814516 * .999999 < const_CSCH; 4.966821568814516 * 1.000001 > const_CSCH
Print "COTH: "; 1.44280551632034 * .999999 < const_COTH; 1.44280551632034 * 1.000001 > const_COTH
Print "COS: "; .9800665778412416 * .999999 < const_COS; .9800665778412416 * 1.000001 > const_COS
Print "SIN: "; .1986693307950612 * .999999 < const_SIN; .1986693307950612 * 1.000001 > const_SIN
Print "TAN: "; .2027100355086725 * .999999 < const_TAN; .2027100355086725 * 1.000001 > const_TAN
Print "EXP: "; 1.22140275816017 * .999999 < const_EXP; 1.22140275816017 * 1.000001 > const_EXP
Print "ATN: "; .1973955598498808 * .999999 < const_ATN; .1973955598498808 * 1.000001 > const_ATN
Print "D2R: "; .0034906585 * .999999 < const_D2R; .0034906585 * 1.000001 > const_D2R
Print "D2G: "; .22222222222 * .999999 < const_D2G; .22222222222 * 1.000001 > const_D2G
Print "R2D: "; 11.4591559 * .999999 < const_R2D; 11.4591559 * 1.000001 > const_R2D
Print "R2G: "; .0031415926 * .999999 < const_R2G; .0031415926 * 1.000001 > const_R2G
Print "G2D: "; .18 * .999999 < const_G2D; .18 * 1.000001 > const_G2D
Print "G2R: "; 12.7323954474 * .999999 < const_G2R; 12.7323954474 * 1.000001 > const_G2R
Print "CSC: "; 1.099750170294616 * .999999 < const_CSC; 1.099750170294616 * 1.000001 > const_CSC
Print "SQR: "; 4.47213595499958 * .999999 < const_SQR; 4.47213595499958 * 1.000001 > const_SQR
Print "ROOT: "; 2.714417616594907 * .999999 < const_ROOT; 2.714417616594907 * 1.000001 > const_ROOT

Print "LOG: "; -1.6094379124341 * .999999 > const_LOG; -1.6094379124341 * 1.000001 < const_LOG
Print "SEC: "; -2.402997961722381 * .999999 > const_SEC; -2.402997961722381 * 1.000001 < const_SEC
Print "COT: "; -.4576575543602858 * .999999 > const_COT; -.4576575543602858 * 1.000001 < const_COT

Print "ABS: "; const_ABS
Print "SGN: "; const_SGN
Print "INT: "; const_INT
Print "ROUND: "; const_ROUND
Print "CEIL: "; const_CEIL
Print "FIX: "; const_FIX
Print "RGB32: "; Hex$(const__RGB32)
Print "RGBA32: "; Hex$(const__RGBA32)
Print "1: "; Hex$(const__RGB32_1)
Print "2: "; Hex$(const__RGB32_2)
Print "4: "; Hex$(const__RGB32_4)
Print "RGB: "; Hex$(const__RGB)
Print "RGBA: "; Hex$(const__RGBA)
Print "RED32: "; Hex$(const__RED32)
Print "GREEN32: "; const__GREEN32
Print "BLUE32: "; const__BLUE32
Print "ALPHA32: "; const__ALPHA32
Print "RED: "; const__RED
Print "GREEN: "; const__GREEN
Print "BLUE: "; const__BLUE
Print "ALPHA: "; const__ALPHA

Print "CHR: "; const_CHR
Print "ASC: "; const_ASC
Print "nested: "; const_nested

System
