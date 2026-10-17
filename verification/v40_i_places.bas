$CONSOLE:ONLY
' Verification (m3-versioninfo, task 1.1): $VERSIONINFO. In a $IF branch not taken, in a SUB, in an included file.
$IF NOSUCHFLAG THEN
    $VERSIONINFO:CompanyName='not taken'
$END IF
'$INCLUDE: 'v40_inc/vi.bi'
s
PRINT "ok"
SHELL "powershell -NoProfile -ExecutionPolicy Bypass -File v40_inc\show_version.ps1 " + CHR$(34) + COMMAND$(0) + CHR$(34)
SYSTEM
SUB s
$VERSIONINFO:ProductName='in a sub'
END SUB
