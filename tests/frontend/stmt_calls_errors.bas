' TEST: check-fail
$CONSOLE:ONLY
' Built-in statement calls rejected (verification\v22_x01-x20): a number for a string ("String required for sub",
' "1st sub argument requires a string"), too few and too many arguments ("Syntax error - Reference: KILL
' fileSpec$"); a built-in statement that is not compiled yet is "not supported yet" at its first word
DIM n AS LONG
KILL 5
KILL
KILL "a", "b"
KILL n
MKDIR 5
CHDIR "a", "b"
RMDIR
ENVIRON 5
ENVIRON "A=1", "B=2"
NAME "a" AS 5
NAME 5 AS "b"
CALL KILL(5)
LOCATE 1, 1
