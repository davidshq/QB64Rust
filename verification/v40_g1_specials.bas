$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. A backslash, a quote inside, an empty value, a value ending in a backslash.
$VERSIONINFO:FileDescription='back\slash'
$VERSIONINFO:Comments='quote ' inside'
$VERSIONINFO:LegalCopyright=''
$VERSIONINFO:LegalTrademarks='ends\'
$VERSIONINFO:InternalName='say "hi"'
$VERSIONINFO:OriginalFilename='a\nb'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
