' TEST: check-fail
$CONSOLE:ONLY
' $VERSIONINFO rejected (m3-versioninfo task 2.1, design D1, D3; verification\v40_x01-x22, v40_h*, v40_g*):
' no key=value after a colon; an unknown key; a numeric key that is not four comma-separated numbers; then
' the numeric parts the old compiler's resource compiler refuses (empty, octal 8 or 9, over 65535); and a <
' or & in the three fields of the manifest, checked at the line kept (the company name set twice, its first
' value bad, compiles; the description's last value is bad)
$VERSIONINFO
$VERSIONINFO:
$VERSIONINFO:=x
$VERSIONINFO:CompanyName
$VERSIONINFO:Author='x'
$VERSIONINFO:FILEVERSION#=
$VERSIONINFO:FILEVERSION#=1,2,3
$VERSIONINFO:FILEVERSION#=1,2,3,4,5
$VERSIONINFO:FILEVERSION#=1,2,3,
$VERSIONINFO:FILEVERSION#=1, 2,3,4
$VERSIONINFO:FILEVERSION#='1,2,3,4'
$VERSIONINFO:FILEVERSION#=a,b,c,d
$VERSIONINFO:FILEVERSION#=-1,2,3,4
$VERSIONINFO;CompanyName='a'
$VERSIONINFO:CompanyName$='a'
$VERSIONINFO:Company Name='a'
$VERSIONINFO:PRODUCTVERSION#=1.2.3.4
$VERSIONINFO:FILEVERSION#=1,2,3,4 ' comment
$VERSIONINFO:FILEVERSION#=1,2,3,"4"
$VERSIONINFO:=
$VERSIONINFO=CompanyName:'a'
$VERSIONINFO:FILEVERSION#=1,,2,3
$VERSIONINFO:FILEVERSION#=,1,2,3
$VERSIONINFO:FILEVERSION#=70000,0,0,99999
$VERSIONINFO:PRODUCTVERSION#=65536,0,0,0
$VERSIONINFO:FILEVERSION#=08,09,010,0
$VERSIONINFO:CompanyName='A & B'
$VERSIONINFO:CompanyName='A and B'
$VERSIONINFO:ProductName='less<than'
$VERSIONINFO:FileDescription='fine'
$VERSIONINFO:FileDescription='a & b'
SYSTEM
