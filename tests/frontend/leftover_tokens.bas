' TEST: check-fail
$CONSOLE:ONLY
' Tokens left after a complete statement are an error, not silently dropped
x = 5 6
y = 5)
DIM a b
DIM c 7
END
