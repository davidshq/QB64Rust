' Screen program (Wars of Napoleon port, step 7b): the BIOS keyboard buffer's head (1050) and tail (1052) as a
' long run of keys moves them. Each key typed moves the tail two bytes on, reading it moves the head, and both
' wrap from slot 60 back to 30: the 16 slots of the buffer. The game's naval combat asks PEEK(1052) = 30, so the
' wrap decides when the check is true again. The keys file gives one key at each announced wait. The last two
' keys are F12, which INKEY$ gives as CHR$(0) + CHR$(134); the port's test hook takes it back out of the
' buffer by setting both bytes one slot back.
DEFINT A-Z
SCREEN 0
DEF SEG = 0
POKE 1050, 30: POKE 1052, 30
PRINT "clear: head"; PEEK(1050); " tail"; PEEK(1052)
FOR i = 1 TO 19
	POKE 1264, (PEEK(1264) + 1) AND 255
	DO
		k$ = INKEY$
	LOOP WHILE k$ = ""
	PRINT "key"; i; ":"; LEN(k$); ASC(RIGHT$(k$, 1)); " head"; PEEK(1050); " tail"; PEEK(1052);
	IF k$ = CHR$(0) + CHR$(134) THEN
		slot = PEEK(1052) - 2
		IF slot < 30 THEN slot = 60
		POKE 1050, slot: POKE 1052, slot
		PRINT " -> head"; PEEK(1050); " tail"; PEEK(1052);
	END IF
	PRINT
NEXT i
DEF SEG
SYSTEM
