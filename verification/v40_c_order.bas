$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. A string version before its numeric key stays; one after replaces the default.
$VERSIONINFO:FileVersion='text first'
$VERSIONINFO:FILEVERSION#=1,2,3,4
$VERSIONINFO:PRODUCTVERSION#=1,0,0,0
$VERSIONINFO:ProductVersion='text after'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
