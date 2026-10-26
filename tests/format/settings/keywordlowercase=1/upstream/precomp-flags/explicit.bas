$if WINDOWS OR MACOSX then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $console:only
    print "OPTION _EXPLICIT active. (Implies _EXPLICITARRAY)"
    print "OPTION _EXPLICITARRAY active."
    print "$ASSERTS inactive."
    print "$CONSOLE active (w/o ONLY option)."
    print "$DEBUG inactive."
    print "DEPENDENCY_SOCKETS inactive."
    system
$end if

'--- this will set _ASSERTS_ = 1
'$ASSERTS
'--- this will set _ASSERTS_ = 1 and _CONSOLE_ = 1
'$ASSERTS:CONSOLE
'--- this will set _CONSOLE_ = 1
$console
'--- this will set/override to _CONSOLE_ = 2
'$CONSOLE:ONLY
'--- this will set _DEBUG_ = 1 and _SOCKETS_ = 1 (as $DEBUG contains _OPENHOST/CLIENT)
'$DEBUG
'--- this will set _SOCKETS_ = 1
'PRINT _OPENCLIENT("http:")
'--- this will set _EXPLICIT_ = 1 and _EXPLICITARRAY_ = 1
option _explicit
'--- this will set _EXPLICITARRAY_ = 1
'OPTION _EXPLICITARRAY

'======================================
$if _CONSOLE_ = 1 then
    _dest _console
$end if
'======================================
$if _EXPLICIT_ then
    print "OPTION _EXPLICIT active. (Implies _EXPLICITARRAY)"
$else
    PRINT "OPTION _EXPLICIT inactive."
$end if
'======================================
$if _EXPLICITARRAY_ then
    print "OPTION _EXPLICITARRAY active."
$else
    PRINT "OPTION _EXPLICITARRAY inactive."
$end if
'======================================
$if _ASSERTS_ then
    PRINT "$ASSERTS active."
$else
    print "$ASSERTS inactive."
$end if
'======================================
$if _CONSOLE_ then
    _dest _console
    print "$CONSOLE active ";
    $if _CONSOLE_ = 1 then
        print "(w/o ONLY option)."
    $elseif _CONSOLE_ = 2 then
        PRINT "(with ONLY option)."
    $end if
$else
    PRINT "$CONSOLE inactive."
$end if
'======================================
$if _DEBUG_ then
    PRINT "$DEBUG active."
$else
    print "$DEBUG inactive."
$end if
'======================================
$if _SOCKETS_ then
    PRINT "DEPENDENCY_SOCKETS active."
$else
    print "DEPENDENCY_SOCKETS inactive."
$end if
'======================================

system
