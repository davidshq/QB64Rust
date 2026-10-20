' TEST: check-fail
$CONSOLE:ONLY
' $EXEICON with a file that is there but is no icon (cpp-failure-errors task 2.2, design D1; verification\v29_e16):
' an error at the line, before any C++, where both compilers' C++ build fails. It names the file as the program
' writes it. The check is the driver's, made for every command that reports diagnostics
DIM l AS LONG
PRINT "before"
$EXEICON:'./inc/not_an_icon.ico'
l = "a"
SYSTEM
