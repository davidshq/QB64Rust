$CONSOLE:ONLY
MKDIR "_files_1_dir"
OPEN "_files_1_dir/a.tmp" FOR OUTPUT AS #1: CLOSE #1
OPEN "_files_1_dir/b.tmp" FOR OUTPUT AS #1: CLOSE #1
n = 0
f$ = _FILES$("_files_1_dir/*.tmp")
DO WHILE LEN(f$)
    n = n + 1
    f$ = _FILES$
LOOP
PRINT "Files found:"; n
KILL "_files_1_dir/a.tmp": KILL "_files_1_dir/b.tmp"
RMDIR "_files_1_dir"
END
