$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. With $EXEICON, the icon line after the version block in the source.
$VERSIONINFO:ProductName='with icon'
$EXEICON:'./v29_inc/icon.ico'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
