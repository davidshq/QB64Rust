# Prints the version resource of the executable it is given (m3-versioninfo, task 1.1), one field a line
# between brackets, and nothing of its path. Started by the v40 programs with SHELL and COMMAND$(0).
param([string]$exe)
$v = (Get-Item -LiteralPath $exe).VersionInfo
foreach ($f in 'FileVersion', 'ProductVersion', 'CompanyName', 'FileDescription', 'InternalName',
    'LegalCopyright', 'LegalTrademarks', 'OriginalFilename', 'ProductName', 'Comments', 'Language') {
    Write-Output ("{0}=[{1}]" -f $f, $v.$f)
}
Write-Output ("FileParts={0},{1},{2},{3}" -f $v.FileMajorPart, $v.FileMinorPart, $v.FileBuildPart, $v.FilePrivatePart)
Write-Output ("ProductParts={0},{1},{2},{3}" -f $v.ProductMajorPart, $v.ProductMinorPart, $v.ProductBuildPart, $v.ProductPrivatePart)
