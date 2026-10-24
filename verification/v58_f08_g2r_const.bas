$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _G2R of a variable of each type beside the CONST of the same literal
' (design D2 asked whether the expression and the CONST evaluator agree; they need not).
CONST c = _G2R(200)
CONST cs! = _G2R(200)
CONST cd# = _G2R(200)
CONST cf## = _G2R(200)
s! = 200: d# = 200: f## = 200: i% = 200
PRINT c; cs!; cd#; cf##
PRINT _G2R(s!); _G2R(d#); _G2R(f##); _G2R(200); _G2R(i%)
PRINT c = _G2R(f##); cf## = _G2R(f##); cd# = _G2R(d#); cs! = _G2R(s!)
SYSTEM
