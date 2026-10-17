$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. Every key once, in quotes.
$VERSIONINFO:CompanyName='Acme Co'
$VERSIONINFO:FileDescription='A test file'
$VERSIONINFO:FILEVERSION#=1,2,3,4
$VERSIONINFO:PRODUCTVERSION#=5,6,7,8
$VERSIONINFO:FileVersion='1.2.3.4 beta'
$VERSIONINFO:InternalName='internal'
$VERSIONINFO:LegalCopyright='(c) 2026'
$VERSIONINFO:LegalTrademarks='TM'
$VERSIONINFO:OriginalFilename='orig.exe'
$VERSIONINFO:ProductName='Product'
$VERSIONINFO:ProductVersion='5.6 prod'
$VERSIONINFO:Comments='a comment'
$VERSIONINFO:Web='https://example.org'
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
