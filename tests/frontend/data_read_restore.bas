' TEST: check-fail
$CONSOLE:ONLY
' DATA, READ and RESTORE parse, and sema marks each "not supported yet" (m2-parser-breadth task 5.2); text after a
' closing quote is a real error, as in the old compiler
DATA 1, "a:b", c REM d: READ x
RESTORE
DATA "a" b
