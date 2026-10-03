' SYNTAX TEST "source.qb64rust" "comment forms"
PRINT 1 ' trailing text
'       ^^^^^^^^^^^^^^^ comment.line.apostrophe.qb64rust
REM whole line
' <--- keyword.other.rem.qb64rust
'   ^^^^^^^^^^ comment.line.rem.qb64rust
rem lower case
' <--- comment.line.rem.qb64rust
    Rem indented
'   ^^^^^^^^^^^^ comment.line.rem.qb64rust
x = 1: REM after colon
'      ^^^^^^^^^^^^^^^ comment.line.rem.qb64rust
10 REM after line number
'  ^^^^^^^^^^^^^^^^^^^^^ comment.line.rem.qb64rust
remark = 1
' <------ - comment
