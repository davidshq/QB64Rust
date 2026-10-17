$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. A key given twice.
$VERSIONINFO:CompanyName='first'
$VERSIONINFO:CompanyName='second'
$VERSIONINFO:FILEVERSION#=1,1,1,1
$VERSIONINFO:FILEVERSION#=2,2,2,2
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
