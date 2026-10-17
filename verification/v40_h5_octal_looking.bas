$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. Parts that read as octal in C.
$VERSIONINFO:FILEVERSION#=08,09,010,0
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
