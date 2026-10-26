$Console:Only
' Runtime comparison: ON n GOSUB line1, line2, ...
On 1 GOSUB S1, S2
Print "after 1"
On 2 GOSUB S1, S2
Print "after 2"
End
S1:
Print "S1"
Return
S2:
Print "S2"
Return
