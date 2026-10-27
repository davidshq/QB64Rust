' TEST: check-fail
$CONSOLE:ONLY
' $EXEICON with a file that is not there (m3-sound task 4.2, design D4; verification\v29_e04): an error at the
' line, counted with the others, that names the file as the program writes it (the old compiler names the folder
' it looked in: QB64PE-DIVERGENCES.md). The check is the driver's, made for every command that reports diagnostics
DIM l AS LONG
PRINT "before"
$EXEICON:'./inc/missing.ico'
l = "a"
SYSTEM
