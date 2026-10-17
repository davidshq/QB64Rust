$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. One part one over 16 bits.
$VERSIONINFO:FILEVERSION#=65536,0,0,0
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
