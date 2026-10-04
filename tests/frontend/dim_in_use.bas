' TEST: check-fail
$CONSOLE:ONLY
' design D5 (verification\v13b_dim_suffix_error): DIM of an existing variable
x& = 3
DIM x AS LONG
DIM y AS LONG, y AS LONG
DIM z% AS INTEGER
DIM w AS INTEGER
DIM w AS LONG
v = 1.5
DIM v
END
