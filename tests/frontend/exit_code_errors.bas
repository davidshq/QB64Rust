' TEST: check-fail
' The forms of END n and SYSTEM n the old compiler rejects (m3-asserts-debug-zlib design D5; verification/v33_x37
' -x39, x43, x48): a string, two values, empty parentheses, a whole TYPE value. Each is an error of the program,
' none "not supported yet"; the messages are new.
$CONSOLE:ONLY
TYPE t
    a AS LONG
END TYPE
DIM v AS t
SYSTEM "a"
END "a"
SYSTEM 1, 2
SYSTEM ()
SYSTEM v
