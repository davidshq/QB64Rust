$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. A longer name that starts with $VERSIONINFO.
$VERSIONINFOX:CompanyName='prefix'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
