$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. Letter case, double quotes, blanks, no quotes, a trailing comment, : and = in a value.
$versioninfo:companyname="double quoted"
$VERSIONINFO : FileDescription = 'spaced'   
$VERSIONINFO:ProductName=no quotes
$VERSIONINFO:Comments='a' ' trailing comment
$VERSIONINFO:InternalName='a:b=c'
$VersionInfo:FileVersion#=1,2,3,4
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
