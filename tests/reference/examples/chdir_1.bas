$CONSOLE:ONLY
MKDIR "chdir_1_dir"
PRINT "Made:"; _DIREXISTS("chdir_1_dir")
CHDIR "chdir_1_dir"
OPEN "inside.tmp" FOR OUTPUT AS #1: CLOSE #1
KILL "inside.tmp"
CHDIR ".."
RMDIR "chdir_1_dir"
PRINT "Removed:"; _DIREXISTS("chdir_1_dir")
END
