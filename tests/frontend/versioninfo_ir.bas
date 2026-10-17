' TEST: ir
$CONSOLE:ONLY
' $VERSIONINFO in the IR (m3-versioninfo task 2.2, design D1, D2; verification\v40_b, v40_c, v40_e, v40_i): the
' settings carry the version resource. A line in a $IF branch not taken sets nothing; one in a SUB and one in
' an included file set their fields. The last line of a key counts; FILEVERSION# gives the string file version
' its digits as FileVersion is still empty, while ProductVersion, set before PRODUCTVERSION#, keeps its text.
' No statement stands where a line stands
$IF NOSUCHFLAG THEN
    $VERSIONINFO:CompanyName='not taken'
$END IF
'$INCLUDE: 'inc/versioninfo.bi'
$VERSIONINFO:CompanyName='first'
$VERSIONINFO:CompanyName='second'
$VERSIONINFO:ProductVersion='text first'
$VERSIONINFO:FILEVERSION#=1,1,1,1
$VERSIONINFO:FILEVERSION#=2,2,2,2
$VERSIONINFO:PRODUCTVERSION#=3,0,0,0
PRINT "ok"
s
SYSTEM
SUB s
$VERSIONINFO:ProductName='in a sub'
END SUB
