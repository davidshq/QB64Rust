$Console:Only

Option _Explicit

Const TEXT_FAIL = "Failed"
Const TEXT_PASS = "Passed"
Const TEXT_ASCII = Chr$(176) + Chr$(177) + Chr$(178) + " " + TEXT_PASS + " " + Chr$(178) + Chr$(177) + Chr$(176)

Print "Clipboard set test: ";

_Clipboard$ = TEXT_FAIL
_Clipboard$ = TEXT_PASS

If TEXT_PASS = _Clipboard$ Then
    Print TEXT_PASS
Else
    Print TEXT_FAIL
End If

Print "Clipboard extended ASCII set test: ";

_Clipboard$ = TEXT_ASCII

If TEXT_ASCII = _Clipboard$ Then
    Print TEXT_PASS
Else
    Print TEXT_FAIL
End If

Print "Clipboard clear test: ";

_Clipboard$ = TEXT_FAIL
_Clipboard$ = ""

If "" = _Clipboard$ Then
    Print TEXT_PASS
Else
    Print TEXT_FAIL
End If

System
