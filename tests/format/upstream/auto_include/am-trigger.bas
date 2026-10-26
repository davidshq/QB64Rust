'Test to prove the "AfterMain" trigger for auto-including works as
'expected. The trigger is set by the first found valid SUB/FUNC in
'the main module or its natural end, if no SUB/FUNC is used.

$Console:Only

$Let PPVAR = -1
$If PPVAR Then
    'SUB/FUNC inside DECLARE blocks should NOT trigger
    Declare Library "dummy"
        Function AnyDumbLibFunc% ()
        Sub AnyDumbLibSub ()
    End Declare
$Else
    'inactive code blocks should generally NOT trigger
    DECLARE LIBRARY "dummy"
        FUNCTION AnyDumbLibFunc% ()
        SUB AnyDumbLibSub ()
    END DECLARE
    'not even outside DECLARE blocks
    FUNCTION TestFunc
        PRINT "Test"
        TestFunc = 0
    END FUNCTION
    SUB TestSub
        PRINT "Test"
    END SUB
$End If

'commented code should generally NOT trigger
'FUNCTION TestFunc
'    PRINT "TestFunc"
'    TestFunc = 0
'END FUNCTION
Rem SUB TestSub
Rem     PRINT "TestSub"
Rem END SUB

'If anything unexpected triggers the "AfterMain" auto-include, then
'the following line will never be executed, either because already
'the compiler failed or because the auto-include logic inserted an
'implicit END before here.
Print "All good, test passed..."

System

