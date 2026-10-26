$console:only

'interfacing internal variables
declare library "errtest"
    function GetErrorHistory$
    function GetErrorGotoLine&
end declare

'testing handler behavior
Status
on error goto mainHandler 'set w/o history entry
Status
error 5
NormalTest
error 5
RecursiveTest 3
error 5
on error goto _lasthandler 'no more history entries => silent fallback to "unhandled"
Status

'return to test script
system

'-----------------------------------------------------------------------
mainHandler:
print " CALLED: main handler"
resume next
'-----
nSubHandler:
print " CALLED: normal sub handler"
resume next
'-----
rSubHandler:
print " CALLED: recursive sub handler"
resume next

'-----------------------------------------------------------------------
sub NormalTest
    on error goto _newhandler nSubHandler 'set with history entry
    Status
    error 5
    on error goto _lasthandler 'restore to the last handler from history
    Status
end sub
'-----
sub RecursiveTest (n%)
    on error goto _newhandler rSubHandler 'set with history entry
    Status
    error 5
    if n% > 1 then RecursiveTest n% - 1
    on error goto _lasthandler 'restore to the last handler from history
    Status
end sub

'-----------------------------------------------------------------------
sub Status
    print: print "HISTORY: "; "<"; GetErrorHistory$; ">"
    select case GetErrorGotoLine&
        case 0: print " ACTIVE: unhandled"
        case 1: print " ACTIVE: main handler"
        case 2: print " ACTIVE: normal sub handler"
        case 3: print " ACTIVE: recursive sub handler"
    end select
end sub

