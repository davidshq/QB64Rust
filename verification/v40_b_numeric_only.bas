$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. Only the two numeric keys: the string versions default to them.
$VERSIONINFO:FILEVERSION#=1,2,3,4
$VERSIONINFO:PRODUCTVERSION#=10,20,30,40
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
