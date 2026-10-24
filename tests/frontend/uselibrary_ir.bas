' TEST: ir
' $USELIBRARY (m4-builtins-remaining design D7; verification\v58_u01, u03, u06): the libraries under the compiler
' root tests\frontend\inc\root\libraries. Their top files come before the main file, the last registered first
' (test/c's top file uses test/a, so a's top file comes before c's), the END of QB64pe's aftermain.bas and the
' after-main files come after the main module, the bottom files after them; a CONST of a top file, a GOSUB
' routine of an after-main file and the SUBs of the bottom files are the program's.
$CONSOLE:ONLY
$USELIBRARY:'test/three'
$USELIBRARY:'test/c'
PRINT "main "; top$; THREE_K
GOSUB three_handler
ThreeSub
CSub
SYSTEM
