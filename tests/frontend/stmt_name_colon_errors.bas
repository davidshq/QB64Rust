' TEST: check-fail
$CONSOLE:ONLY
' A built-in statement's name and a colon where the statement needs an argument: an error, as in the old compiler
' ("Syntax error - Reference: PSET ...", verification\v27_x59-x62); a jump to such a name finds no label ("Invalid
' label", x63). A built-in that is not compiled yet keeps its mark
DIM l AS LONG
PSET: l = 1
LINE: l = 2
DRAW: l = 3
PCOPY: l = 4
_LIMIT: l = 5
KILL: l = 6
CIRCLE: l = 7
SWAP: l = 8
CLS: l = 9
GOTO CLS
FILES: l = 10
SYSTEM
