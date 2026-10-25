' TEST: debug-info
$CONSOLE:ONLY
' The debug symbol file of a program with an included file (m5-debug-build-symbols task 2.2, spec
' compiler/debug-info "An included file"): the file is listed, and its FUNCTION and its variable name it.
'$INCLUDE:'inc/debug_info.bi'
PRINT twice&(3); counted
