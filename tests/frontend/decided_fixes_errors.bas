' TEST: check-fail
$CONSOLE:ONLY
' A `/` by 0 in a CONST is a compile error (DIVERGENCES.md D-007, m2-numeric-types design D8; the old compiler gives
' 0), also inside a longer expression; `\` and MOD by 0 were errors already (`const_errors.bas`).
CONST c = 1 / 0
CONST d = 2 + 4 / (3 - 3)
SYSTEM
