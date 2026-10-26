$Console:Only
' Runtime comparison: _STRCMP, _STRICMP
Print _StrCmp("abc", "abc")
Print _StrCmp("abc", "abd")
Print _StrCmp("abd", "abc")
Print _StriCmp("ABC", "abc")
Print _StriCmp("aBc", "AbC")
End
