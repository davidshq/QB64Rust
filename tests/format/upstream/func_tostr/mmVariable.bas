$Console:Only

smi! = -3.402823E+38
sma! = 3.402823E+38
dmi# = -1.797693134862315D+308
dma# = 1.797693134862315D+308
fmi## = -1.189731495357231765F+4932
fma## = 1.189731495357231765F+4932
Print
Print "("; _ToStr$(smi!); ")"
Print "("; _ToStr$(sma!); ")"
Print
Print "("; _ToStr$(dmi#); ")"
Print "("; _ToStr$(dma#); ")"
Print
$If _ARM_ Then
    ' ARM64 does not have hardware FP80 support
    PRINT "(-1.189731495357231765F+4932)"
    PRINT "(1.189731495357231765F+4932)"
$Else
    Print "("; _ToStr$(fmi##); ")"
    Print "("; _ToStr$(fma##); ")"
$End If
System

