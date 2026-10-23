' TEST: check-fail
$CONSOLE:ONLY
' $EXEICON rejected (m3-sound task 4.1, design D4; verification\v29_e05-e07, e13, e17, e19, e23, e27, e29): no
' name between single quotes; after the first good line every further one, whatever it names, also inside an
' included file; after a colon and after a tab, as any metacommand. Marked "not supported yet": a drive with no
' slash after it, which the old compiler looks for in the folder it stands in on that drive (m4-devices-continue-paths
' design D3; a relative name without ./ is looked for under the compiler root, crates\driver\tests\cli.rs)
DIM l AS LONG
$EXEICON
$EXEICON:''
$EXEICON:"./inc/exeicon.ico"
$EXEICON:'./inc/exeicon.ico
$EXEICON:'C:exeicon.ico'
$EXEICON:'./inc/exeicon.ico'
$EXEICON:'./inc/exeicon.ico'
$EXEICON:'./inc/missing.ico'
$EXEICON
'$INCLUDE:'inc/exeicon.bi'
l = 1: $EXEICON:'./inc/exeicon.ico'
	$EXEICON:'./inc/exeicon.ico'
SYSTEM
