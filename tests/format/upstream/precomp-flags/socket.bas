$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print "OPTION _EXPLICIT inactive."
    Print "OPTION _EXPLICITARRAY inactive."
    Print "$ASSERTS inactive."
    Print "$CONSOLE active (w/o ONLY option)."
    Print "$DEBUG inactive."
    Print "DEPENDENCY_SOCKETS active."
    System
$End If

'--- this will set _ASSERTS_ = 1
'$ASSERTS
'--- this will set _ASSERTS_ = 1 and _CONSOLE_ = 1
'$ASSERTS:CONSOLE
'--- this will set _CONSOLE_ = 1
$Console
'--- this will set/override to _CONSOLE_ = 2
'$CONSOLE:ONLY
'--- this will set _DEBUG_ = 1 and _SOCKETS_ = 1 (as $DEBUG contains _OPENHOST/CLIENT)
'$DEBUG
'--- this will set _SOCKETS_ = 1
Print _OpenClient("http:")
'--- this will set _EXPLICIT_ = 1 and _EXPLICITARRAY_ = 1
'OPTION _EXPLICIT
'--- this will set _EXPLICITARRAY_ = 1
'OPTION _EXPLICITARRAY

'======================================
$If _CONSOLE_ = 1 Then
    _Dest _Console
$End If
'======================================
$If _EXPLICIT_ Then
    PRINT "OPTION _EXPLICIT active. (Implies _EXPLICITARRAY)"
$Else
    Print "OPTION _EXPLICIT inactive."
$End If
'======================================
$If _EXPLICITARRAY_ Then
    PRINT "OPTION _EXPLICITARRAY active."
$Else
    Print "OPTION _EXPLICITARRAY inactive."
$End If
'======================================
$If _ASSERTS_ Then
    PRINT "$ASSERTS active."
$Else
    Print "$ASSERTS inactive."
$End If
'======================================
$If _CONSOLE_ Then
    _Dest _Console
    Print "$CONSOLE active ";
    $If _CONSOLE_ = 1 Then
        Print "(w/o ONLY option)."
    $ElseIf _CONSOLE_ = 2 Then
        PRINT "(with ONLY option)."
    $End If
$Else
    PRINT "$CONSOLE inactive."
$End If
'======================================
$If _DEBUG_ Then
    PRINT "$DEBUG active."
$Else
    Print "$DEBUG inactive."
$End If
'======================================
$If _SOCKETS_ Then
    Print "DEPENDENCY_SOCKETS active."
$Else
    PRINT "DEPENDENCY_SOCKETS inactive."
$End If
'======================================

System
