'--- this will set _ASSERTS_ = 1
$Asserts
'--- this will set _ASSERTS_ = 1 and _CONSOLE_ = 1
$Asserts:Console
'--- this will set _CONSOLE_ = 1
$Console
'--- this will set/override to _CONSOLE_ = 2
$Console:Only
'--- this will set _DEBUG_ = 1 and _SOCKETS_ = 1 (as $DEBUG contains _OPENHOST/CLIENT)
$Debug
'--- this will set _SOCKETS_ = 1
Print _OpenClient("http:")
'--- this will set _EXPLICIT_ = 1 and _EXPLICITARRAY_ = 1
Option _Explicit
'--- this will set _EXPLICITARRAY_ = 1
Option _ExplicitArray

'======================================
$If _CONSOLE_ = 1 Then
    _DEST _CONSOLE
$End If
'======================================
$If _EXPLICIT_ Then
    Print "OPTION _EXPLICIT active. (Implies _EXPLICITARRAY)"
$Else
    PRINT "OPTION _EXPLICIT inactive."
$End If
'======================================
$If _EXPLICITARRAY_ Then
    Print "OPTION _EXPLICITARRAY active."
$Else
    PRINT "OPTION _EXPLICITARRAY inactive."
$End If
'======================================
$If _ASSERTS_ Then
    Print "$ASSERTS active."
$Else
    PRINT "$ASSERTS inactive."
$End If
'======================================
$If _CONSOLE_ Then
    _Dest _Console
    Print "$CONSOLE active ";
    $If _CONSOLE_ = 1 Then
        PRINT "(w/o ONLY option)."
    $ElseIf _CONSOLE_ = 2 Then
        Print "(with ONLY option)."
    $End If
$Else
    PRINT "$CONSOLE inactive."
$End If
'======================================
$If _DEBUG_ Then
    Print "$DEBUG active."
$Else
    PRINT "$DEBUG inactive."
$End If
'======================================
$If _SOCKETS_ Then
    Print "DEPENDENCY_SOCKETS active."
$Else
    PRINT "DEPENDENCY_SOCKETS inactive."
$End If
'======================================

System

