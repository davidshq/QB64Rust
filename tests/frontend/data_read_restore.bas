' TEST: check-fail
$CONSOLE:ONLY
' DATA, READ and RESTORE parse (m2-parser-breadth task 5.2) and compile (m2-builtin-statements group 4); text after
' a closing quote is a real error, as in the old compiler
DATA 1, "a:b", c REM d: READ x
RESTORE
DATA "a" b
