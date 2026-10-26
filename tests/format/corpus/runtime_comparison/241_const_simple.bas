$Console:Only
' Runtime comparison: CONST with basic math (subset of QB64pe const/expression; no unsigned/EQV/IMP)
Const cadd = 2 + 3
Const csub = 2 - 3
Const cmul = 7 * 20
Const cdiv = 6 / 3
Const cidiv = 7 \ 3
Const cmod = 20 Mod 3
Const cpow = 3 ^ 4
Const cneg = -20
Const ceq = 2 = 2
Const cgt = 3 > 2
Const cstr = "foobar"
Print cadd; csub; cmul; cdiv; cidiv; cmod; cpow; cneg
Print ceq; cgt
Print cstr
End
