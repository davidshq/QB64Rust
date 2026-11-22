' TEST: call-trace
' A call-trace build: each procedure pushes its frame first and pops it at exit_subfunc; a statement that calls a
' procedure of the program stores its site before it, and one that calls none, or calls only a built-in, stores
' nothing. Each GOSUB's site is in a table. A handler pushes its own frame in mainerr.txt; RESUME NEXT takes it
' off, and RESUME to a label of the main module drops every frame, as RUN does. A SUB run by ON TIMER is called
' with site 1, an event; a label run by ON KEY runs in a frame of the main module's. The main module's start marks
' its thread. The tables and the run-time part follow the program's globals.
$CONSOLE:ONLY
ON ERROR GOTO trap
ON TIMER(1) tick
ON KEY(1) GOSUB pressed
x = 1
show x
y = twice&(x) + LEN("ab")
GOSUB work
IF y THEN RUN
SYSTEM
work:
RETURN
pressed:
RETURN
trap:
IF ERR = 5 THEN RESUME again
RESUME NEXT
again:
SYSTEM
SUB show (n)
    PRINT n
END SUB
SUB tick
    IF twice&(1) > 9 THEN EXIT SUB
END SUB
FUNCTION twice& (n)
    twice& = 2 * n
END FUNCTION
