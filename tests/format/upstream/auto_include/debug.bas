'Test to prove vWatch availability and the pre-compiler state flag.

$Debug
$Console:Only

$If _DEBUG_ Then
    Print "Compiling with $DEBUG features enabled."
$Else
    PRINT "Compiling without $DEBUG features."
$End If

System

