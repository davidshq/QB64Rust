$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _R2G of a variable of each type beside the CONST of the same literal
' (design D2 asked whether the expression and the CONST evaluator agree; they need not).
CONST c = _R2G(1)
CONST cs! = _R2G(1)
CONST cd# = _R2G(1)
CONST cf## = _R2G(1)
s! = 1: d# = 1: f## = 1: i% = 1
PRINT c; cs!; cd#; cf##
PRINT _R2G(s!); _R2G(d#); _R2G(f##); _R2G(1); _R2G(i%)
PRINT c = _R2G(f##); cf## = _R2G(f##); cd# = _R2G(d#); cs! = _R2G(s!)
SYSTEM
