$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): PRINT USING with strings and with several values. The
' fields !, & and \ \, a string given to a number field and a number to a string field (errors), a format used
' again when values are left, a format with no field, an empty format, the separators , and ; between values and
' at the end, the cursor after the statement, a format that is a variable or an expression, values that are
' expressions and function calls, TAB and SPC among the values, USING after other items of a PRINT (not after a
' comma: PRINT with a comma never ends when the old compiler's console is redirected, study/00 section 5).
ON ERROR GOTO h
DIM f AS STRING, x AS STRING * 5, n AS LONG
PRINT "string fields"
PRINT "["; : PRINT USING "!"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "!"; ""; : PRINT "]"
PRINT "["; : PRINT USING "&"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "&"; ""; : PRINT "]"
PRINT "["; : PRINT USING "\\"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "\ \"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "\  \"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "\      \"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "\  \"; ""; : PRINT "]"
PRINT "["; : PRINT USING "\ab\"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "\"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "!\"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "a&b"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "_!&"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "_&&"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "_\\ \"; "hello"; : PRINT "]"
x = "ab": PRINT "["; : PRINT USING "&"; x; : PRINT "]"
x = "ab": PRINT "["; : PRINT USING "\      \"; x; : PRINT "]"
PRINT "the wrong kind of value"
PRINT "["; : PRINT USING "###"; "hello"; : PRINT "]"
PRINT "["; : PRINT USING "&"; 5; : PRINT "]"
PRINT "["; : PRINT USING "!"; 5; : PRINT "]"
PRINT "["; : PRINT USING "\  \"; 5; : PRINT "]"
PRINT "["; : PRINT USING "### &"; 5; 6; : PRINT "]"
PRINT "["; : PRINT USING "& ###"; "a"; "b"; : PRINT "]"
PRINT "["; : PRINT USING "ab ### cd &"; 5; 6; : PRINT "]"
PRINT "several values"
PRINT "["; : PRINT USING "## ## ##"; 1; 2; 3; : PRINT "]"
PRINT "["; : PRINT USING "## ## ##"; 1; 2; : PRINT "]"
PRINT "["; : PRINT USING "## ## ##"; 1; : PRINT "]"
PRINT "["; : PRINT USING "##"; 1; 2; 3; : PRINT "]"
PRINT "["; : PRINT USING "##,"; 1; 2; 3; : PRINT "]"
PRINT "["; : PRINT USING "a##b"; 1; 2; 3; : PRINT "]"
PRINT "["; : PRINT USING "a##b##c"; 1; 2; 3; : PRINT "]"
PRINT "["; : PRINT USING "##x"; 1, 2, 3; : PRINT "]"
PRINT "["; : PRINT USING "& is ## years"; "Ann"; 30; "Bob"; 41; : PRINT "]"
PRINT "["; : PRINT USING "& is ## years"; "Ann"; 30; "Bob"; : PRINT "]"
PRINT "["; : PRINT USING "x & y"; "a"; : PRINT "]"
PRINT "["; : PRINT USING "## tail"; 1; : PRINT "]"
PRINT "["; : PRINT USING "head ## mid ## tail"; 1; : PRINT "]"
PRINT "["; : PRINT USING "head ## mid _# tail"; 1; : PRINT "]"
PRINT "no field"
PRINT "["; : PRINT USING "abc"; 1; : PRINT "]"
PRINT "["; : PRINT USING ""; 1; : PRINT "]"
PRINT "["; : PRINT USING "abc"; "x"; : PRINT "]"
PRINT "["; : PRINT USING "_#"; 1; : PRINT "]"
PRINT "the end of the statement"
PRINT USING "##"; 1
PRINT USING "##"; 1;
PRINT "|"
PRINT USING "##"; 1,
PRINT "|"
PRINT USING "##"; 1; 2
PRINT USING "##"; 1, 2
PRINT USING "## text"; 1
PRINT "the cursor"
PRINT USING "###"; 1; : PRINT POS(0)
PRINT USING "###"; 1, : PRINT POS(0)
PRINT "the format"
f = "<##>": PRINT USING f; 5
PRINT USING f + f; 5; 6
PRINT USING LEFT$("##.##xyz", 5); 3.14159
PRINT USING (f); 5
PRINT USING STRING$(3, "#") + ".#"; 5
x = "##": PRINT "["; : PRINT USING x; 5; : PRINT "]"
PRINT "values"
n = 7
PRINT USING "### ###"; n + 1; n * 2
PRINT USING "### ###"; ABS(-5); (n)
PRINT USING "### &"; LEN("abcd"); MID$("abcdef", 2, 3)
PRINT USING "###"; (1), (2)
PRINT "TAB and SPC"
PRINT USING "## ##"; 1; TAB(10); 2
PRINT USING "## ##"; 1; SPC(3); 2
PRINT USING "##"; TAB(5); 1
PRINT USING "## ## ##"; 1; TAB(10); 2; 3
PRINT "USING after other items"
PRINT "a"; USING "##"; 5
PRINT 1; 2; USING "##"; 5; 6
PRINT "a"; "b" USING "##"; 5
PRINT TAB(5); USING "##"; 5
PRINT ; USING "##"; 5
PRINT "an error in a value"
PRINT USING "## ## ##"; 1; 1 / 0; 3
PRINT "after"
PRINT USING "## & ##"; 1; MID$("abc", 0); 3
PRINT "after"
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
