# Runs the sandbox family's executables inside Windows Sandbox (tools\sandbox\README.md; OpenSpec change
# desktop-sandbox, design D3 step 4). The .wsb that run.py writes starts it at logon, from C:\run, the host's
# target\sandbox\run mapped read-write.
#
#   C:\run\list.txt      the programs' names, one per line, in the order to run them
#   C:\run\timeout.txt   the limit of one program, in seconds
#   C:\run\bin\<name>.exe
#
# Each program is copied to its own folder C:\run\results\<name>\ and run there, where it writes out.txt, with its coverage
# counts (QB64RUST_COVERAGE) in that folder's counts. Its exit code, or "timeout" when it was ended at the limit,
# goes to that folder's exit. Then C:\run\results\done is written and the sandbox shuts down.

$run = 'C:\run'
$results = Join-Path $run 'results'
$limit = [int](Get-Content (Join-Path $run 'timeout.txt') -TotalCount 1)
New-Item -ItemType Directory -Force $results | Out-Null

foreach ($name in Get-Content (Join-Path $run 'list.txt')) {
    if (-not $name) { continue }
    $dir = Join-Path $results $name
    New-Item -ItemType Directory -Force $dir | Out-Null
    $env:QB64RUST_COVERAGE = Join-Path $dir 'counts'
    try {
        # A QB64 program makes its executable's folder the current one at its start: each runs from its own folder.
        $exe = Join-Path $dir "$name.exe"
        Copy-Item (Join-Path $run "bin\$name.exe") $exe -ErrorAction Stop
        $p = Start-Process -FilePath $exe -WorkingDirectory $dir -PassThru -ErrorAction Stop
        # Windows PowerShell 5.1 gives no ExitCode for a process started this way unless its handle was taken
        # while it ran.
        $null = $p.Handle
        if ($p.WaitForExit($limit * 1000)) {
            $code = "$($p.ExitCode)"
        } else {
            Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
            $code = 'timeout'
        }
    } catch {
        $code = "not started: $($_.Exception.Message)"
    }
    Set-Content -Path (Join-Path $dir 'exit') -Value $code -Encoding ascii
}

Set-Content -Path (Join-Path $results 'done') -Value 'done' -Encoding ascii
shutdown /s /t 0
