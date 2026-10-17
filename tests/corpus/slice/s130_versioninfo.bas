$CONSOLE:ONLY
' Slice (m3-versioninfo, task 3.2, design D5; from verification\v40_a_all_keys, v40_c_order, v40_d_forms,
' v40_e_repeat, v40_g8_amp_in_comments, v40_h3_leading_zeros): the executable's own version resource, read back by
' a PowerShell script the program writes, runs on COMMAND$(0) with SHELL and deletes. Every key that .NET's
' FileVersionInfo shows, FILEVERSION# set (without it every string reads as empty); letter case, double quotes,
' blanks around the key, a : and = in a value, a key set twice, the string file version taken from the digits,
' octal-looking parts, & and < outside the manifest's fields. Nothing of the path is printed
$VERSIONINFO:CompanyName='Acme Co'
$versioninfo:filedescription="double quoted"
$VERSIONINFO : FILEVERSION# = 1,2,3,4
$VERSIONINFO:PRODUCTVERSION#=5,06,010,0
$VERSIONINFO:InternalName='a:b=c'
$VERSIONINFO:LegalCopyright='(c) 2026'
$VERSIONINFO:LegalTrademarks='TM'
$VERSIONINFO:OriginalFilename='orig.exe'
$VERSIONINFO:ProductName='first'
$VERSIONINFO:ProductName='Product'
$VERSIONINFO:ProductVersion='5.6 prod'
$VERSIONINFO:Comments='A & B <c>'
$VERSIONINFO:Web='https://example.org'
DIM q AS STRING
q = CHR$(39)
OPEN "s130.ps1" FOR OUTPUT AS #1
PRINT #1, "param([string]$exe)"
PRINT #1, "$v = (Get-Item -LiteralPath $exe).VersionInfo"
PRINT #1, "foreach ($f in " + q + "FileVersion" + q + ", " + q + "ProductVersion" + q + ", " + q + "CompanyName" + q + ", ";
PRINT #1, q + "FileDescription" + q + ", " + q + "InternalName" + q + ", " + q + "LegalCopyright" + q + ", ";
PRINT #1, q + "LegalTrademarks" + q + ", " + q + "OriginalFilename" + q + ", " + q + "ProductName" + q + ", ";
PRINT #1, q + "Comments" + q + ") {"
PRINT #1, "    Write-Output (" + q + "{0}=[{1}]" + q + " -f $f, $v.$f)"
PRINT #1, "}"
PRINT #1, "Write-Output (" + q + "FileParts={0},{1},{2},{3}" + q + " -f $v.FileMajorPart, $v.FileMinorPart, ";
PRINT #1, "$v.FileBuildPart, $v.FilePrivatePart)"
PRINT #1, "Write-Output (" + q + "ProductParts={0},{1},{2},{3}" + q + " -f $v.ProductMajorPart, $v.ProductMinorPart, ";
PRINT #1, "$v.ProductBuildPart, $v.ProductPrivatePart)"
CLOSE #1
PRINT "before"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File s130.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
KILL "s130.ps1"
PRINT "after"
SYSTEM
