$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _D2G of a variable of each type beside the CONST of the same literal
' (design D2 asked whether the expression and the CONST evaluator agree; they need not).
CONST c = _D2G(90)
CONST cs! = _D2G(90)
CONST cd# = _D2G(90)
CONST cf## = _D2G(90)
s! = 90: d# = 90: f## = 90: i% = 90
PRINT c; cs!; cd#; cf##
PRINT _D2G(s!); _D2G(d#); _D2G(f##); _D2G(90); _D2G(i%)
PRINT c = _D2G(f##); cf## = _D2G(f##); cd# = _D2G(d#); cs! = _D2G(s!)
SYSTEM
