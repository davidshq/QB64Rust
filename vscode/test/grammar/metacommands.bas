' SYNTAX TEST "source.qb64rust" "metacommands"
'$INCLUDE:'lib.bi'
' <- punctuation.definition.comment.qb64rust
' <- - comment
'^^^^^^^^ keyword.control.directive.metacommand.qb64rust
'         ^^^^^^^^ string.quoted.single.qb64rust
' $DYNAMIC
' ^^^^^^^^ keyword.control.directive.metacommand.qb64rust
REM $STATIC
' <--- keyword.other.rem.qb64rust
'   ^^^^^^^ keyword.control.directive.metacommand.qb64rust
'   ^^^^^^^ - comment
$CONSOLE
' <-------- keyword.control.directive.metacommand.qb64rust
$If WIN Then
' <--- keyword.control.directive.metacommand.qb64rust
' plain comment
' <--------------- comment.line.apostrophe.qb64rust
