$Console:Only

Print
Print "("; _ToStr$(_SINGLE_MIN); ")"
Print "("; _ToStr$(_SINGLE_MAX); ")"
Print
Print "("; _ToStr$(_DOUBLE_MIN); ")"
Print "("; _ToStr$(_DOUBLE_MAX); ")"
Print
$If _ARM_ Then
    ' ARM64 does not have hardware FP80 support
    PRINT "(-1.189731495357231765F+4932)"
    PRINT "(1.189731495357231765F+4932)"
$Else
    Print "("; _ToStr$(_FLOAT_MIN); ")"
    Print "("; _ToStr$(_FLOAT_MAX); ")"
$End If
System

