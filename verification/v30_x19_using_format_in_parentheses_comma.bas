$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (19_using_format_in_parentheses_comma)
PRINT USING MID$("##.##", 1, 2); 5
PRINT "ok"
SYSTEM
