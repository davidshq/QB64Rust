$If WINDOWS OR MACOSX Then
  ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
  $Console:Only
  Print "OPTION _EXPLICIT active. (Implies _EXPLICITARRAY)"
  Print "OPTION _EXPLICITARRAY active."
  Print "$ASSERTS inactive."
  Print "$CONSOLE active (w/o ONLY option)."
  Print "$DEBUG inactive."
  Print "DEPENDENCY_SOCKETS inactive."
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
'PRINT _OPENCLIENT("http:")
'--- this will set _EXPLICIT_ = 1 and _EXPLICITARRAY_ = 1
Option _Explicit
'--- this will set _EXPLICITARRAY_ = 1
'OPTION _EXPLICITARRAY

'======================================
$If _CONSOLE_ = 1 Then
  _Dest _Console
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
    PRINT "DEPENDENCY_SOCKETS active."
$Else
  Print "DEPENDENCY_SOCKETS inactive."
$End If
'======================================

System
