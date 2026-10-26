$Console:Only
' Slice (m3-console-files, task 5.3; from verification\v30_i_lock): LOCK and UNLOCK, 11 numbered lines. The three
' forms with and without the number sign, a comma alone, variables; record 0 and below (error 5), a range the wrong
' way round (75), an UNLOCK that matches no LOCK and a second LOCK of the same bytes (70); the program reads and
' writes what it locked; a second OPEN of the file inside the program cannot read it (70); CLOSE ends the lock;
' every mode takes it; a file that is not open (52); after CALL. One program, one process: nothing here needs a
' second program. The file is s85_file_locks.tmp and is removed.
On Error GoTo handler
Dim f As String, c As String * 1, n As Long
f = "s85_file_locks.tmp"
Open f For Output As #1: Print #1, "abcdefghij": Close #1
Open f For Binary As #1
Print "1:";: Lock #1: Unlock #1: Lock 1: Unlock 1: Lock #1, 2: Unlock #1, 2: Lock #1, 2 To 4: Unlock #1, 2 To 4: Print " forms"
n = 1
Print "2:";: Lock #1,: Unlock #1,: Lock #n, n To n + 2: Unlock #n, n To n + 2: Call Lock(1, 2 To 3): Call Unlock( #1, 2 To 3): Print " more forms"
Print "3:";: Lock #1, 0: Unlock #1, 0: Lock #1, -1: Print
Print "4:";: Lock #1, 4 To 2: Unlock #1, 4 To 2: Print
Print "5:";: Lock #1, 100 To 200: Unlock #1, 100 To 200: Print " past the end"
Print "6:";: Unlock #1: Lock #1, 2 To 4: Unlock #1, 2 To 3: Unlock #1, 2 To 4: Print
Print "7:";: Lock #1: Lock #1: Unlock #1: Unlock #1: Print
Print "8:";: Lock #1: Get #1, 1, c: Print " "; c;: Put #1, 1, c: Unlock #1: Print
Print "9:";: Lock #1
Open f For Binary As #2: c = "?": Get #2, 1, c: Print " "; c;: Close #2: Unlock #1
Lock #1: Close #1
Open f For Binary As #1: c = "?": Get #1, 1, c: Print " "; c: Close #1
Open f For Random As #1 Len = 2
Print "10:";: Lock #1: Unlock #1: Lock #1, 2: Unlock #1, 2: Lock #1, 2 To 3: Unlock #1, 2 To 3: Close #1
Open f For Input As #1: Lock #1: Unlock #1: Lock #1, 2: Unlock #1, 2: Close #1
Open f For Append As #1: Lock #1: Unlock #1: Close #1
Open f For Output As #1: Lock #1: Print #1, "x": Unlock #1: Close #1
Print " every mode"
Print "11:";: Lock #3: Unlock #3: Lock #0: Print
Kill f
System
handler:
Print " <error"; Err; ">";
Resume Next
