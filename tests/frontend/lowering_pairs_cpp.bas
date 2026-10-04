' TEST: cpp
$CONSOLE:ONLY
' Lowering pairs (design D6, FreeBASIC lesson L12): source line -> IR -> C++
' PRINT item forms
PRINT "x="; a; INSTR(3, s$, "l")
PRINT "a" 1; x "b";
PRINT
PRINT s$ + "!"; -a; 1.5; 1 / 3
' an assignment with each conversion kind
i% = 7: l& = i%: i% = l&
d# = 2.5: x% = d#: l& = d#: q&& = d#
s! = d#: d# = 1: s! = i%: f## = s!
' INSTR without and with the optional argument
PRINT INSTR(s$, "l"); INSTR(3, s$, "l"); INSTR(2.6, s$, "l")
END
