' TEST: ir
' A program with neither $CONSOLE nor $CONSOLE:ONLY is compiled as one with a window and no console
' (m3-window-output task 2.1, design D1): the settings line says so; PRINT and END are the calls they are in a
' console program
PRINT 1
END
