'$ENCODING:UTF8
$CONSOLE:ONLY
' Verification (utf8-source, task 1.1): a file saved as UTF-8 and declared UTF-8. QB64pe copies the
' three bytes of the box-drawing character into the string.
PRINT LEN("─"); ASC("─")
SYSTEM
