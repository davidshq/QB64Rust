$Console:Only
' Slice (m3-versioninfo, task 3.2, design D5; from verification\v40_a_all_keys, v40_c_order, v40_d_forms,
' v40_e_repeat, v40_g8_amp_in_comments, v40_h3_leading_zeros): the executable's own version resource, read back by
' a PowerShell script the program writes, runs on COMMAND$(0) with SHELL and deletes. Every key that .NET's
' FileVersionInfo shows, FILEVERSION# set (without it every string reads as empty); letter case, double quotes,
' blanks around the key, a : and = in a value, a key set twice, the string file version taken from the digits,
' octal-looking parts, & and < outside the manifest's fields. Nothing of the path is printed
$VersionInfo:CompanyName='Acme Co'
$VersionInfo:FileDescription='double quoted'
$VersionInfo:FILEVERSION#=1,2,3,4
$VersionInfo:PRODUCTVERSION#=5,06,010,0
$VersionInfo:InternalName='a:b=c'
$VersionInfo:LegalCopyright='(c) 2026'
$VersionInfo:LegalTrademarks='TM'
$VersionInfo:OriginalFilename='orig.exe'
$VersionInfo:ProductName='first'
$VersionInfo:ProductName='Product'
$VersionInfo:ProductVersion='5.6 prod'
$VersionInfo:Comments='A & B <c>'
$VersionInfo:Web='https://example.org'
Dim q As String
q = Chr$(39)
Open "s130.ps1" For Output As #1
Print #1, "param([string]$exe)"
Print #1, "$v = (Get-Item -LiteralPath $exe).VersionInfo"
Print #1, "foreach ($f in " + q + "FileVersion" + q + ", " + q + "ProductVersion" + q + ", " + q + "CompanyName" + q + ", ";
Print #1, q + "FileDescription" + q + ", " + q + "InternalName" + q + ", " + q + "LegalCopyright" + q + ", ";
Print #1, q + "LegalTrademarks" + q + ", " + q + "OriginalFilename" + q + ", " + q + "ProductName" + q + ", ";
Print #1, q + "Comments" + q + ") {"
Print #1, "    Write-Output (" + q + "{0}=[{1}]" + q + " -f $f, $v.$f)"
Print #1, "}"
Print #1, "Write-Output (" + q + "FileParts={0},{1},{2},{3}" + q + " -f $v.FileMajorPart, $v.FileMinorPart, ";
Print #1, "$v.FileBuildPart, $v.FilePrivatePart)"
Print #1, "Write-Output (" + q + "ProductParts={0},{1},{2},{3}" + q + " -f $v.ProductMajorPart, $v.ProductMinorPart, ";
Print #1, "$v.ProductBuildPart, $v.ProductPrivatePart)"
Close #1
Print "before"
Shell "powershell -NoProfile -ExecutionPolicy Bypass -File s130.ps1 " + Chr$(34) + Command$(0) + Chr$(34)
Kill "s130.ps1"
Print "after"
System
