' TEST: check-fail
$CONSOLE:ONLY
' The forms of $ASSERTS, $ASSERTS:CONSOLE, $DEBUG and _ASSERT the old compiler rejects (m3-asserts-debug-zlib
' design D1, D3, task 2.1; verification/v33_x05, x07-x11, x16-x22, x24, x47; a tab after them is in
' console_meta_tab_after.bas). Each is an error of the program, none "not supported yet"; the messages are new.
$ASSERTS x
$ASSERTS:CONSOLEX
$ASSERTS:
$ASSERTS:ONLY
$ASSERTS : CONSOLE
$DEBUG x
x = 1: $ASSERTS
x = 1: $DEBUG
TYPE t
    a AS LONG
END TYPE
DIM v AS t
_ASSERT
_ASSERT "a"
_ASSERT 1, 5
_ASSERT 1, "a", 2
_ASSERT 1,
_ASSERT , "a"
CALL _ASSERT
_ASSERT v
SYSTEM
