' TEST: ir
$CONSOLE:ONLY
' Built-in statements in the IR (m2-builtin-statements task 1.5, design D2): Op::Builtin names the table entry, never
' a libqb name, with one slot per argument or choice in template order; a statement with one value (KILL, MKDIR,
' RMDIR, CHDIR, ENVIRON), with two values and its mandatory word (NAME), and in a procedure; each may raise
DIM s AS STRING
KILL "f.tmp"
MKDIR s
RMDIR s + "d"
CHDIR ".."
NAME s AS s + "2"
ENVIRON "A=" + s
remove s
SUB remove (f AS STRING)
    KILL f
    NAME f AS "g"
END SUB
