' TEST: check-fail
' $USELIBRARY's errors (m4-builtins-remaining design D7; verification\v58_u12-u16): the name without quotes, in
' double quotes or empty, a library with no descriptor under the compiler root (tests\frontend\inc\root), and a
' descriptor whose IncAfterMain file is missing; each at its line, none "not supported yet".
$CONSOLE:ONLY
$USELIBRARY:test/a
$USELIBRARY:"test/a"
$USELIBRARY:''
$USELIBRARY:'test/none'
$USELIBRARY:'test/broken'
PRINT "ok"
