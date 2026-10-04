' TEST: typed
$CONSOLE:ONLY
' design D5 (verification\v13_dim_suffix): a plain DIM after DIM AS changes nothing; DIM AS after a plain DIM retypes
DIM m AS LONG
DIM m
m = 2.5
DIM n
DIM n AS LONG
n = 2.5
END
