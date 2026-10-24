$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _ACOSH of a variable of each type and of a literal (the CONST evaluator rejects
' it, v58_x107: no CONST here), stored into each type.
s! = 1.5: d# = 1.5: f## = 1.5: i% = 1
PRINT _ACOSH(s!); _ACOSH(d#); _ACOSH(f##); _ACOSH(1.5); _ACOSH(i%)
rs! = _ACOSH(f##): rd# = _ACOSH(f##): rf## = _ACOSH(f##): rl& = _ACOSH(f##)
PRINT rs!; rd#; rf##; rl&
SYSTEM
