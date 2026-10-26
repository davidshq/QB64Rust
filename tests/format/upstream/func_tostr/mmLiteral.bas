$Console:Only

Print
Print "("; _ToStr$(-3.402823E+38); ")"
Print "("; _ToStr$(3.402823E+38); ")"
Print
Print "("; _ToStr$(-1.797693134862315D+308); ")"
Print "("; _ToStr$(1.797693134862315D+308); ")"
Print
$If _ARM_ Then
    ' ARM64 does not have hardware FP80 support
    PRINT "(-1.189731495357231765F+4932)"
    PRINT "(1.189731495357231765F+4932)"
$Else
    Print "("; _ToStr$(-1.189731495357231765F+4932); ")"
    Print "("; _ToStr$(1.189731495357231765F+4932); ")"
$End If
System

