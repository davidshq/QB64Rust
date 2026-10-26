$Console:Only

h& = _OpenClient("https://www.example.com")
Print h&

length&& = LOF(h&)

result$ = ""
While Not EOF(h&)
    _Limit 20
    Get #h&, , s$
    result$ = result$ + s$
Wend


' Strip off the trailing slash if it's there to make the result consistent
url$ = _ConnectionAddress$(h&)
If Mid$(url$, Len(url$), 1) = "/" Then
    url$ = Left$(url$, Len(url$) - 1)
End If

Print "Content-Length: "; _IIf(length&& >= 0, length&&, Len(result$))
Print "Url: "; url$
Print "Status Code: "; _StatusCode(h&)
Print
Print result$

Close h&

System
