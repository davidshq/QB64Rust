' TEST: check-fail
$CONSOLE:ONLY
' The old compiler's stubs are its "Command not implemented", one per statement, also where never reached and after
' CALL (m4-statements-metacommands, design D4; verification\v57_s01-s20, v32_x89-x91). RESET and CALL CLOSE compile.
OPEN "stubs.tmp" FOR OUTPUT AS #1
RESET
CALL CLOSE
CALL CLOSE(1)
CALL RESET
TRON
TROFF
LIST
IOCTL 1, "x"
x$ = IOCTL$(1)
x = SETMEM(0)
x = FILEATTR(1, 1)
x = FRE(0)
x = FRE("")
CALL TRON
CALL FRE(0)
a$ = "abc"
CALL MID$(a$, 1, 1, "x")
DIM m AS _MEM
CALL _MEMPUT(m, m.OFFSET, 1 AS LONG)
RESET 1
