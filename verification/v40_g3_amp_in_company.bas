$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. An & in the company name, which stands in the manifest's assembly name.
$VERSIONINFO:CompanyName='A & B'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
