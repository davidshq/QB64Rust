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
# goes to that folder's exit. Then C:\run\results\done is written.
#
# -Shutdown then shuts the machine down, as Windows Sandbox needs. In the test VM (tools\winvm\run.ps1; OpenSpec
# change windows-vm, design D5) the script runs without it, and the host turns the VM off.
#
# -OpenGL <folder> is given in the test VM, whose display has no OpenGL that can make a QB64 program's window. The
# folder's DLLs, Mesa's software OpenGL, are copied beside each program for its run and removed after it, and Mesa
# is told to use llvmpipe.

param([switch]$Shutdown, [string]$OpenGL)

$run = 'C:\run'
$results = Join-Path $run 'results'
$limit = [int](Get-Content (Join-Path $run 'timeout.txt') -TotalCount 1)
New-Item -ItemType Directory -Force $results | Out-Null
$dlls = if ($OpenGL) { @(Get-ChildItem -LiteralPath $OpenGL -Filter *.dll) } else { @() }
if ($OpenGL) { $env:GALLIUM_DRIVER = 'llvmpipe' }

foreach ($name in Get-Content (Join-Path $run 'list.txt')) {
    if (-not $name) { continue }
    $dir = Join-Path $results $name
    New-Item -ItemType Directory -Force $dir | Out-Null
    $env:QB64RUST_COVERAGE = Join-Path $dir 'counts'
    try {
        # A QB64 program makes its executable's folder the current one at its start: each runs from its own folder.
        $exe = Join-Path $dir "$name.exe"
        Copy-Item (Join-Path $run "bin\$name.exe") $exe -ErrorAction Stop
        foreach ($dll in $dlls) { Copy-Item -LiteralPath $dll.FullName $dir -ErrorAction Stop }
        $p = Start-Process -FilePath $exe -WorkingDirectory $dir -PassThru -ErrorAction Stop
        # Windows PowerShell 5.1 gives no ExitCode for a process started this way unless its handle was taken
        # while it ran.
        $null = $p.Handle
        if ($p.WaitForExit($limit * 1000)) {
            $code = "$($p.ExitCode)"
        } else {
            Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
            # Until it has ended, its files are in use and cannot be removed.
            $null = $p.WaitForExit(5000)
            $code = 'timeout'
        }
    } catch {
        $code = "not started: $($_.Exception.Message)"
    }
    foreach ($dll in $dlls) { Remove-Item -LiteralPath (Join-Path $dir $dll.Name) -ErrorAction SilentlyContinue }
    Set-Content -Path (Join-Path $dir 'exit') -Value $code -Encoding ascii
}

Set-Content -Path (Join-Path $results 'done') -Value 'done' -Encoding ascii
if ($Shutdown) { shutdown /s /t 0 }
