' TEST: check-fail
$CONSOLE:ONLY
' Template mismatches (m2-parser-breadth task 7.5, design D6): each line alone is rejected by the old compiler
' ("Syntax error - Reference: …", checked with `qb64pe.exe -z` one line per program, 2026-10-07); here each gets
' one error, at the furthest token a form reached, naming the forms.
LINE (0, 0) (9, 9)
LINE (0, 0)-(9, 9), 1, XX
PSET 1, 2
CIRCLE (1, 1)
OPEN "f" FOR INPUT #1
LOCATE 1, 2, 3, 4, 5, 6
SCREEN 12 13
