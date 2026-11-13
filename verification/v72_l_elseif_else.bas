$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): statements after ELSEIF ... THEN and after the ELSE of a block IF.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 2
c = 0
IF a = 0 THEN
x = 1
ELSEIF a = 1 THEN x = 2: c = 3
ELSE x = 3
END IF
PRINT x; c
NEXT
SYSTEM
