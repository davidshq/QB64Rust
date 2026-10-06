' TEST: ir
$CONSOLE:ONLY
' The operators that may raise make their statement may_raise (m2-control-flow-slice D4): \ and MOD by 0
' (error 11), ^ with a negative base (error 5); a comparison or logic on numbers does not
DIM l AS LONG, s AS SINGLE
l = l \ 2
l = l MOD 2
s = s ^ 2
l = l < 2
l = l AND 2
l = l _ANDALSO 2
END
