$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _G2D of a variable of each type beside the CONST of the same literal
' (design D2 asked whether the expression and the CONST evaluator agree; they need not).
CONST c = _G2D(100)
CONST cs! = _G2D(100)
CONST cd# = _G2D(100)
CONST cf## = _G2D(100)
s! = 100: d# = 100: f## = 100: i% = 100
PRINT c; cs!; cd#; cf##
PRINT _G2D(s!); _G2D(d#); _G2D(f##); _G2D(100); _G2D(i%)
PRINT c = _G2D(f##); cf## = _G2D(f##); cd# = _G2D(d#); cs! = _G2D(s!)
SYSTEM
