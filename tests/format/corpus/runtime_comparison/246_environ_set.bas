$Console:Only
' Runtime comparison: ENVIRON set then ENVIRON$ read (QB64pe compile_tests/environ style)
Environ "RT246_FOO=BAR"
Print Environ$("RT246_FOO")
Environ "RT246_X=ZZZ"
Print Environ$("RT246_X")
Print "[";
Print Environ$("RT246_NOSUCH");
Print "]"
End
