' TEST: debug-info
$CONSOLE:ONLY
' A SUB opened here and closed in an included file, as the old compiler reads included text (include_cross_ok.bas):
' the procedure's end_file is the included file's index, its file this one's (m5-debug-build-symbols task 3.4).
show
SUB show
'$INCLUDE:'inc/debug_info_end.bi'
