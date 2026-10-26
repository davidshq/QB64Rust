$console:only
' Slice (m5b-format-settings, task 1.1): the text forms of the format oracle under the other layout settings
' (tests\format\settings.list): an unclosed string at the end of a line, '$FORMAT:OFF and '$FORMAT:ON, ?, joined
' lines, REM, DATA, and spacing written by hand.
print "question mark"
print "no space"; 1; 2
a = 1: b = 2: c = a + b: print a; b; c
print "unclosed at the end"
x$ = "also unclosed:   kept as written"
print x$
'$format:off
    if a=1   then
  print   "format off: as written"
        end if
'$format:on
if a = 1 then
    print "format on again"
end if
rem    a REM line
print a: rem a REM after a statement
read d1, d2$
print d1; d2$
data 12,hello there,"x"
print using "##.##"; 3.14159
print chr$(65) + lcase$("BC"); len("abc")
s$ = string$(3, "*"): print s$
if not (a = 2 and b = 2) or c = 3 then print "logic"
y# = -a ^ 2 + b mod 2 - c \ 2: print y#
