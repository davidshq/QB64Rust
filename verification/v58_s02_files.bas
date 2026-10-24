$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): FILES of a pattern, bare (the current folder), of a folder, of a pattern in a
' folder, of an empty string, of a pattern that matches nothing (error 53 after the path), of a folder that does
' not exist, and of a folder name with a trailing backslash, in the family's folder v58_fil (CHDIR there first,
' so that the listing is the folder's four entries). The path line and the free space are the machine's: the
' compare masks them (tests\verification\normalize\v58_s02_files.normalize).
ON ERROR GOTO handler
CHDIR "v58_fil"
FILES "*.txt"
PRINT "--"
FILES
PRINT "--"
FILES "sub"
PRINT "--"
FILES "sub\*.txt"
PRINT "--"
FILES ""
PRINT "--"
FILES "nosuch.*"
PRINT "--"
FILES "nosuchdir\*.txt"
PRINT "--"
FILES "sub\"
PRINT "--"
FILES "*.dat"
PRINT "done"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
