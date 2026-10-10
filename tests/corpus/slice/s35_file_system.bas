$CONSOLE:ONLY
' Built-in statements that are one call of the runtime (spec language/builtin-statements, language/file-io
' "File-system statements"): MKDIR, CHDIR, NAME, RMDIR, KILL and ENVIRON. Their effect is shown by the errors a
' second call raises under a handler (the file functions come with the files, s38): 75 for a folder that exists, 76
' for one that does not, 53 for a missing file, 5 for a raising argument (once: the first error is the one serviced)
' and for an ENVIRON text without a separator. RESUME runs a failing statement and its arguments again. Every folder
' is named s35_* and removed again.
ON ERROR GOTO handler
DIM SHARED handled AS LONG, calls AS LONG
DIM d AS STRING, tries AS LONG
d = "s35_dir"
PRINT "MKDIR"
MKDIR d
PRINT "MKDIR again"
MKDIR d
PRINT "CHDIR into it and back"
CHDIR d
MKDIR "s35_inner"
CHDIR ".."
PRINT "RMDIR of a folder that is not empty"
RMDIR d
PRINT "RMDIR inner, by an expression"
RMDIR d + "/s35_inner"
PRINT "NAME"
NAME d AS d + "2"
PRINT "MKDIR of the old name works again"
MKDIR d
PRINT "NAME onto an existing folder"
NAME d AS d + "2"
PRINT "RMDIR both"
RMDIR d
RMDIR (d + "2")
PRINT "RMDIR again"
RMDIR d
PRINT "CHDIR missing"
CHDIR "s35_none"
PRINT "NAME missing"
NAME "s35_none" AS "s35_other"
PRINT "KILL missing"
KILL "s35_none.tmp"
CALL KILL("s35_none.tmp")
PRINT "KILL empty name"
KILL ""
PRINT "raising arguments"
KILL CHR$(-1)
MKDIR "s35_" + CHR$(-1)
NAME CHR$(-1) AS "s35_x"
NAME "s35_x" AS CHR$(-1)
PRINT "ENVIRON"
ENVIRON "S35_VAR=seven"
ENVIRON "S35_VAR2 eight"
ENVIRON "S35_VAR3"
ENVIRON ""
PRINT "in a SUB"
insub "s35_none.tmp"
PRINT "RESUME runs the statement again"
tries = 0
RMDIR dirname$
PRINT "the FUNCTION ran"; calls; "times"
PRINT "handled"; handled
SYSTEM
handler:
handled = handled + 1
PRINT "  error"; ERR
IF ERR = 76 AND tries = 0 AND calls = 1 THEN
    tries = 1
    MKDIR "s35_again"
    RESUME
END IF
RESUME NEXT

SUB insub (f AS STRING)
    KILL f
    PRINT "  after KILL in the SUB"
    MKDIR "s35_sub"
    MKDIR "s35_sub"
    RMDIR "s35_sub"
    PRINT "  end of the SUB"
END SUB

FUNCTION dirname$
    calls = calls + 1
    dirname$ = "s35_again"
END FUNCTION
