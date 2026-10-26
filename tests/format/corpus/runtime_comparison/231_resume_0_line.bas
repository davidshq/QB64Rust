$Console:Only
' Runtime comparison 231: RESUME 0 (retry) and RESUME <label>
' Part 1: RESUME 0 retries the line that caused the error.
' Part 2: RESUME after_label jumps to the given label.
' Note: With inline runtime, RESUME 0 may not retry (_qb_error_line not set); see DIFFERENCES if results differ from QB64pe.
On Error GoTo handler
Print "part1_start"
flag = 0
If flag = 0 Then Error 5
Print "part1_after_resume0"
GoTo part2
handler:
Print "handler"
If flag = 0 Then flag = 1: Resume 0
Resume after_label
part2:
Print "part2_trigger"
Error 6
Print "part2_after_error"
End
after_label:
Print "after_label"
End
