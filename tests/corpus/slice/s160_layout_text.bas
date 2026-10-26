$CONSOLE:ONLY
' Slice (m5b-format-settings, task 1.1): the text forms of the format oracle under the other layout settings
' (tests\format\settings.list): an unclosed string at the end of a line, '$FORMAT:OFF and '$FORMAT:ON, ?, joined
' lines, REM, DATA, and spacing written by hand.
? "question mark"
?"no space";1;2
a=1:b =2:   c= a+b  :PRINT a;b;c
PRINT "unclosed at the end
x$ = "also unclosed:   kept as written
PRINT x$
'$FORMAT:OFF
    IF a=1   THEN
  print   "format off: as written"
        end if
'$FORMAT:ON
   IF a = 1 THEN
  PRINT "format on again"
        END IF
REM    a REM line
PRINT a   :   REM a REM after a statement
READ d1,d2$
PRINT d1;d2$
DATA   12 ,  hello there  ,"x"
PRINT  USING   "##.##";3.14159
PRINT   CHR$(65)+  LCASE$("BC")  ;  LEN( "abc" )
s$=STRING$(3,"*") : PRINT s$
IF NOT (a = 2 AND b = 2) OR c = 3 THEN PRINT "logic"
y# = -a ^ 2 + b MOD 2 - c \ 2: PRINT y#
