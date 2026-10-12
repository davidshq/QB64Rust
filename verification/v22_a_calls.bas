$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Statement calls"): the file-system statements and ENVIRON under a handler:
' the error of a failing call, of a raising argument (is the call still made, which error is serviced), and the
' statement inside a procedure. Every file and folder is named v22_a_* and removed again.
ON ERROR GOTO h
DIM n AS LONG
PRINT "KILL missing"
KILL "v22_a_none.tmp"
PRINT "KILL raising argument"
KILL CHR$(-1)
PRINT "KILL raising argument + text"
KILL "v22_a_" + CHR$(-1)
PRINT "KILL empty name"
KILL ""
OPEN "v22_a_f.tmp" FOR OUTPUT AS #1: PRINT #1, "x": CLOSE #1
PRINT "KILL existing"
KILL "v22_a_f.tmp"
PRINT "KILL again"
KILL "v22_a_f.tmp"
PRINT "MKDIR"
MKDIR "v22_a_dir"
PRINT "MKDIR again"
MKDIR "v22_a_dir"
PRINT "CHDIR into and back"
CHDIR "v22_a_dir"
CHDIR ".."
PRINT "CHDIR missing"
CHDIR "v22_a_nodir"
PRINT "RMDIR"
RMDIR "v22_a_dir"
PRINT "RMDIR again"
RMDIR "v22_a_dir"
PRINT "MKDIR empty name"
MKDIR ""
PRINT "NAME missing"
NAME "v22_a_none.tmp" AS "v22_a_other.tmp"
OPEN "v22_a_g.tmp" FOR OUTPUT AS #1: PRINT #1, "x": CLOSE #1
PRINT "NAME existing"
NAME "v22_a_g.tmp" AS "v22_a_h.tmp"
PRINT "NAME raising first argument"
NAME CHR$(-1) AS "v22_a_i.tmp"
PRINT "NAME raising second argument"
NAME "v22_a_h.tmp" AS CHR$(-1)
PRINT "NAME both raising"
NAME CHR$(-1) AS LEFT$("a", -1)
PRINT "KILL renamed"
KILL "v22_a_h.tmp"
PRINT "ENVIRON with ="
ENVIRON "V22_A_VAR=seven"
PRINT "["; ENVIRON$("V22_A_VAR"); "]"
PRINT "ENVIRON with a blank"
ENVIRON "V22_A_VAR2 eight"
PRINT "["; ENVIRON$("V22_A_VAR2"); "]"
PRINT "ENVIRON without = or blank"
ENVIRON "V22_A_VAR3"
PRINT "ENVIRON empty"
ENVIRON ""
PRINT "ENVIRON raising argument"
ENVIRON CHR$(-1)
PRINT "in a SUB"
inproc "v22_a_none.tmp"
PRINT "expression arguments"
DIM f AS STRING
f = "v22_a_k"
MKDIR f + "dir"
RMDIR f + "dir"
PRINT "handler ran"; n; "times"
SYSTEM
h:
n = n + 1
PRINT "  error"; ERR
RESUME NEXT

SUB inproc (f AS STRING)
    KILL f
    PRINT "  after KILL in SUB"
    MKDIR "v22_a_subdir"
    RMDIR "v22_a_subdir"
    RMDIR "v22_a_subdir"
    PRINT "  end of SUB"
END SUB
