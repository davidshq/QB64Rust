$Embed:'./test.output','test'

$Console
$ScreenHide
_Dest _Console

dat$ = _Embedded$("test")
For i% = 1 To Len(dat$)
    If Asc(dat$, i%) = 10 Or Asc(dat$, i%) = 13 Then Exit For
    Print Mid$(dat$, i%, 1);
Next i%

System

