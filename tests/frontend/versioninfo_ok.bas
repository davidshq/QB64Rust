' TEST: check-ok
$CONSOLE:ONLY
' $VERSIONINFO accepted (m3-versioninfo task 2.1, design D1, D3; verification\v40_a-v40_l, v40_g*, v40_h*, x20):
' every key, letter case, double quotes, blanks, no quotes and a trailing comment (warnings, not errors), a
' : and = in a value, the $VERSIONINFOX prefix, a string FILEVERSION that is not four numbers, the largest
' parts and octal ones, > and UTF-8 in the manifest's fields, & and < in another field
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
$versioninfo:companyname="double quoted"
$VERSIONINFO : FileDescription = 'spaced'
$VERSIONINFO:ProductName=no quotes
$VERSIONINFO:Comments='a' ' trailing comment
$VERSIONINFO:InternalName='a:b=c'
$VersionInfo:FileVersion#=1,2,3,4
$VERSIONINFOX:CompanyName='prefix'
$VERSIONINFO:FILEVERSION=1,2,3,4,5
$VERSIONINFO:FILEVERSION#=65535,65535,65535,65535
$VERSIONINFO:PRODUCTVERSION#=01,02,010,0004
$VERSIONINFO:ProductName='a>b'
$VERSIONINFO:FileDescription='café'
$VERSIONINFO:Comments='A & B <c>'
$VERSIONINFO:LegalCopyright=''
PRINT "ok"
SYSTEM
