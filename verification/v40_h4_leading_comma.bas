$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. A leading comma: three commas and a last digit.
$VERSIONINFO:FILEVERSION#=,1,2,3
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
