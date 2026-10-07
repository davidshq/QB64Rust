' TEST: check-fail
$CONSOLE:ONLY
' OPTION _EXPLICIT inside a SUB applies to the whole program: main's implicit `y` is an error
' (m2-control-flow-slice D7, verification\v17_f_explicit_in_sub_main_implicit); the SUB's DIM is fine.
y = 1
PRINT y
s
SYSTEM

SUB s
    OPTION _EXPLICIT
    DIM v
    v = 1
END SUB
