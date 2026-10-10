$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "The rest"): RANDOMIZE without a seed, with the seed on standard input
' (v22_e_randomize.stdin: 5, then 70000, then abc and 6): the prompt, the value taken, the sequence after it.
ON ERROR GOTO h
PRINT "seed 5"
RANDOMIZE
PRINT RND; RND
RANDOMIZE USING 5
PRINT "USING 5:"; RND; RND
PRINT "seed 70000 (outside -32768 to 32767)"
RANDOMIZE
PRINT RND
PRINT "seed abc, then 6"
RANDOMIZE
PRINT RND
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
