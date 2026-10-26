'Test to prove vWatch availability and the pre-compiler state flag.

$Console:Only

$If _DEBUG_ Then
    PRINT "Compiling with $DEBUG features enabled."
$Else
    Print "Compiling without $DEBUG features."
$End If

System

