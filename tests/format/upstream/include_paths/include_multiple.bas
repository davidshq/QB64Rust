$Console:Only

' Include a file that itself includes another file. It does so in a relative
' fashion

Dim includeCount As Long

'$Include:'../extra/include_extra_include.bi'
'$Include:'./tests/compile_tests/extra/include_extra_include.bi'
'$Include:'tests/compile_tests/extra/include_extra_include.bi'

System
