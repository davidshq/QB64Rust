$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): a relative $EMBED name found under the compiler's folder.
' The runner puts v47_files/text.txt under the old compiler's folder for the recording (v47_e02_embed_root.root).
$EMBED:'v47_files/text.txt','h'
PRINT LEN(_EMBEDDED$("h")); _EMBEDDED$("h")
SYSTEM
