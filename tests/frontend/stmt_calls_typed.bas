' TEST: typed
$CONSOLE:ONLY
' Built-in statements that are one call of the runtime (m2-builtin-statements task 1.4, verification\v22_a_*): the
' six plain ones, each with its table entry and one slot per argument or choice; a string argument as it is (a
' literal, a variable, an expression, a fixed-length string, an element, in parentheses); the CALL form; in a SUB
DIM s AS STRING
DIM fx AS STRING * 8
DIM a(2) AS STRING
KILL "f.tmp"
KILL s
KILL s + ".x"
KILL fx
KILL a(1)
KILL (s)
CALL KILL("f.tmp")
MKDIR "d"
RMDIR s + "d"
CHDIR ".."
NAME s AS s + "2"
name "a" as "b"
ENVIRON "A=1"
remove "x"
SUB remove (f AS STRING)
    KILL f
    MKDIR f + "d"
END SUB
