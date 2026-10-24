$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _ASINH of a variable of each type and of a literal (the CONST evaluator rejects
' it, v58_x107: no CONST here), stored into each type.
s! = 1.5: d# = 1.5: f## = 1.5: i% = 1
PRINT _ASINH(s!); _ASINH(d#); _ASINH(f##); _ASINH(1.5); _ASINH(i%)
rs! = _ASINH(f##): rd# = _ASINH(f##): rf## = _ASINH(f##): rl& = _ASINH(f##)
PRINT rs!; rd#; rf##; rl&
SYSTEM
