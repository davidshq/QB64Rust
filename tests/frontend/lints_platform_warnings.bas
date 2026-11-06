' TEST: warnings
' The lints and the `$IF` branches a compile skips (spec `compiler/lints`, "Code that may run after all"). A name
' that a skipped branch uses may be used on another platform or with other flags. `WinSetup`, `LinuxSetup`, `GoWin`
' and `GoLinux` get no warning, on any platform: each is named in the branch of its platform. `extra` is set once
' and read in a branch `FULL` switches off: no warning. `Lonely` and `stray` (lines 22 and 24) are named in no
' branch, and get theirs. The output is the same on every platform.
$CONSOLE:ONLY
$IF WIN THEN
    WinSetup
    GOTO GoWin
$ELSE
    LinuxSetup
    GOTO GoLinux
$END IF
GoWin:
GoLinux:
$LET FULL = 0
$IF FULL = 1 THEN
    PRINT extra
$END IF
extra = 5
SUB Lonely
    PRINT "never"
    PRINT stray
END SUB
SUB WinSetup
    PRINT "win"
END SUB
SUB LinuxSetup
    PRINT "linux"
END SUB
