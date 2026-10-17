$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. UTF-8 in the product name and description.
$VERSIONINFO:FILEVERSION#=1,0,0,0
$VERSIONINFO:FileDescription='café'
$VERSIONINFO:ProductName='café'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
