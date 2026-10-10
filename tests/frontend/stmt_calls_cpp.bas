' TEST: cpp
$CONSOLE:ONLY
' Built-in statements as C++ (m2-builtin-statements task 1.6, design D5; the old compiler's lines read with
' qb64pe -z, verification\v22_a_args): sub_kill(qbs_new_txt_len("f.tmp",5)), sub_kill(__STRING_S),
' sub_kill(qbs_add(...)), sub_name(a,b), sub_environ(s), each followed by the cleanup of string temporaries; the call
' is made without a test for a pending error (libqb's entry tests)
DIM s AS STRING
KILL "f.tmp"
KILL s
KILL s + ".x"
KILL CHR$(-1)
MKDIR s
RMDIR s
CHDIR s
NAME s AS s + "2"
ENVIRON s
remove s
SUB remove (f AS STRING)
    KILL f
END SUB
