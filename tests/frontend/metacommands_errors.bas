' TEST: check-fail
$CONSOLE:ONLY
' Metacommands the old compiler does not know are its "Syntax error", one per line (m4-statements-metacommands, design
' D3; verification\v16_m6_unknown_meta, v24_x38, v39_x74, v57_m01-m08); $NOPREFIX is its own error (v43_x32);
' $ERRORLOCATION with a word it does not take is "Syntax error" too (v58_e15); a line in a branch not taken is nothing.
$NOSUCHTHING
$nosuch
$FOO:BAR
$STATIC
$DYNAMIC
$INCLUDE: 'inc/data.bi'
$
$MIDISOUNDFONT
$MIDISOUNDFONTS:DEFAULT
$NOPREFIX
$ERRORLOCATION:MAYBE
$IF 0 THEN
$NOSUCHTHING
$END IF
PRINT "ok"
