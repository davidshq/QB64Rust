$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _ATANH of a variable of each type and of a literal (the CONST evaluator rejects
' it, v58_x107: no CONST here), stored into each type.
s! = .5: d# = .5: f## = .5: i% = 1
PRINT _ATANH(s!); _ATANH(d#); _ATANH(f##); _ATANH(.5); _ATANH(i%)
rs! = _ATANH(f##): rd# = _ATANH(f##): rf## = _ATANH(f##): rl& = _ATANH(f##)
PRINT rs!; rd#; rf##; rl&
SYSTEM
