$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. The largest 16-bit parts.
$VERSIONINFO:FILEVERSION#=65535,65535,65535,65535
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
