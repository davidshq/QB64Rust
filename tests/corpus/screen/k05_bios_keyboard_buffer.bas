' Screen program (Civil War Battleset port, step 3): the BIOS keyboard buffer in segment 0. Bytes 1050
' and 1052 (&H41A, &H41C) are the head and the tail of the buffer. The game's BuffClear sets both to 30, the start
' of the buffer, to throw away keys typed ahead, and its automatic play stops when PEEK(1052) > 30, a key waiting.
' The keys file gives one key at each announced wait. The program waits until the tail moves, without reading
' the key, then either sets both bytes to 30 or reads the key with INKEY$, and prints what it found.
DEFINT A-Z
SCREEN 0
DEF SEG = 0
PRINT "start: head"; PEEK(1050); " tail"; PEEK(1052)
FOR phase = 1 TO 2
	POKE 1264, (PEEK(1264) + 1) AND 255
	t! = TIMER
	DO
		_LIMIT 200
	LOOP UNTIL PEEK(1052) > 30 OR TIMER - t! > 5
	PRINT "phase"; phase; ": a key waits, head"; PEEK(1050); " tail"; PEEK(1052)
	IF phase = 1 THEN
		POKE 1050, 30: POKE 1052, 30
		k$ = INKEY$
		PRINT "  after POKE 1050, 30: POKE 1052, 30, INKEY$ gives "; CHR$(34); k$; CHR$(34)
	ELSE
		k$ = INKEY$
		PRINT "  INKEY$ gives "; CHR$(34); k$; CHR$(34); ", then head"; PEEK(1050); " tail"; PEEK(1052)
	END IF
NEXT phase
DEF SEG
SYSTEM
