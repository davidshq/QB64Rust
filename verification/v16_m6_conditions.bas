$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $IF conditions with =, <>, <, >, <=, >=, AND, OR, XOR, bare names,
' predefined names, $ELSEIF, $LET redefinition, letter case of names and values.
$LET A = 1
$LET S = hello
$IF A = 1 THEN
PRINT "A = 1: true"
$ELSE
PRINT "A = 1: false"
$END IF
$IF A <> 1 THEN
PRINT "A <> 1: true"
$ELSE
PRINT "A <> 1: false"
$END IF
$IF A < 2 THEN
PRINT "A < 2: true"
$END IF
$IF A > 0 THEN
PRINT "A > 0: true"
$END IF
$IF A >= 1 THEN
PRINT "A >= 1: true"
$END IF
$IF A <= 0 THEN
PRINT "A <= 0: true"
$ELSE
PRINT "A <= 0: false"
$END IF
$IF A = 1 AND S = hello THEN
PRINT "A = 1 AND S = hello: true"
$END IF
$IF A = 2 OR S = HELLO THEN
PRINT "A = 2 OR S = HELLO: true"
$ELSE
PRINT "A = 2 OR S = HELLO: false"
$END IF
$IF A = 1 XOR S = hello THEN
PRINT "A = 1 XOR S = hello: true"
$ELSE
PRINT "A = 1 XOR S = hello: false"
$END IF
$IF A THEN
PRINT "bare A: true"
$END IF
$IF a = 1 THEN
PRINT "lower-case a = 1: true"
$ELSE
PRINT "lower-case a = 1: false"
$END IF
$IF A = 2 THEN
PRINT "elseif chain: 2"
$ELSEIF A = 1 THEN
PRINT "elseif chain: 1"
$ELSE
PRINT "elseif chain: else"
$END IF
$LET A = 2
$IF A = 2 THEN
PRINT "after $LET A = 2: A = 2 true"
$END IF
$IF WIN THEN
PRINT "WIN"
$END IF
$IF WINDOWS = -1 THEN
PRINT "WINDOWS = -1"
$END IF
$IF LINUX OR MAC OR MACOSX THEN
PRINT "LINUX/MAC"
$END IF
$IF 64BIT THEN
PRINT "64BIT"
$END IF
$IF 32BIT THEN
PRINT "32BIT"
$END IF
$IF _QB64PE_ THEN
PRINT "_QB64PE_"
$END IF
$IF _ARM_ THEN
PRINT "_ARM_"
$END IF
$IF VERSION > 3.0 THEN
PRINT "VERSION > 3.0"
$END IF
$IF VERSION >= 4.7.0 THEN
PRINT "VERSION >= 4.7.0"
$END IF
$IF VERSION < 99 THEN
PRINT "VERSION < 99"
$END IF
$IF B = DEFINED THEN
PRINT "B = DEFINED: true"
$ELSE
PRINT "B = DEFINED: false"
$END IF
$IF B = UNDEFINED THEN
PRINT "B = UNDEFINED: true"
$END IF
$IF A = DEFINED THEN
PRINT "A = DEFINED: true"
$END IF
$IF NOPE THEN
PRINT "undefined bare name: true"
$ELSE
PRINT "undefined bare name: false"
$END IF
SYSTEM
