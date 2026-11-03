# Does one run of the desktop family in the test VM (tools\winvm\README.md; OpenSpec change windows-vm, design D4,
# D6). tools\sandbox\run.py calls it in place of opening Windows Sandbox:
#
#   tools\winvm\run.ps1 -Folder <run folder> -Limit <seconds>
#   tools\winvm\run.ps1 -Check      only the refusals that need no VM started
#   tools\winvm\run.ps1 -Stop       only turns the VM off: run.py's cleanup when it had to kill a run
#
# The run folder is the one run.py prepares: bin\<name>.exe, list.txt, timeout.txt and inside.ps1. The script
# restores the checkpoint "ready", starts the VM and opens a PowerShell Direct session as "tester". It checks the
# guest's clock and license, and copies those files to the guest's C:\run. Then it runs inside.ps1 in tester's
# logged-on desktop session through a scheduled task, and waits up to -Limit seconds for C:\run\results\done.
# It copies C:\run\results\ back into the run folder's results\, also after a timeout. The VM is turned off at the
# end, whatever happened.
#
# Exit 0: done was written. Exit 1: the limit passed, or the run failed after it started; the results it could copy
# are in the folder. Exit 2: refused, with a message saying what to do; the VM is left as it was found.

param(
    [Parameter(ParameterSetName = 'Run', Mandatory = $true)][string]$Folder,
    [Parameter(ParameterSetName = 'Run', Mandatory = $true)][int]$Limit,
    # Only the refusals that need no VM started: exit 0 or 2, and the VM is not touched. run.py checks so before it
    # builds anything.
    [Parameter(ParameterSetName = 'Check', Mandatory = $true)][switch]$Check,
    # Only turns the VM off, as a run's finally does: run.py uses it when it killed this script past its limit, which
    # skips that finally. Exit 0 when the VM is off afterwards, 1 when not.
    [Parameter(ParameterSetName = 'Stop', Mandatory = $true)][switch]$Stop
)

$ErrorActionPreference = 'Stop'
$name = if ($env:QB64RUST_VM) { $env:QB64RUST_VM } else { 'qb64rust-desktop' }
$credentialFile = Join-Path $env:LOCALAPPDATA 'qb64rust\winvm\credential.xml'
$inside = 'C:\run'
# Mesa's software OpenGL in the guest, put there by create.ps1, which inside.ps1 puts beside each program.
$openGL = 'C:\mesa'
$taskName = 'qb64rust-run'
# The guest's Windows license (SoftwareLicensingProduct's ApplicationID for Windows).
$windowsApplication = '55c92734-d682-4d71-983e-d6ec3f16059f'
$startLimit = 120

function Refuse([string]$message) {
    Write-Host "winvm: refused: $message"
    exit 2
}

function Seconds([datetime]$since) { ((Get-Date) - $since).TotalSeconds }

# The refusals that leave the VM untouched (design D6).
$make = 'make it with tools\winvm\create.ps1 (tools\winvm\README.md), or run the family in Windows Sandbox ' +
    'with run.py --where sandbox'
try {
    Import-Module Hyper-V -ErrorAction Stop
    $vm = Get-VM -Name $name -ErrorAction SilentlyContinue
    # Get-VM fails outright, rather than finding nothing, for a user who may not use Hyper-V.
    $null = Get-VMHost -ErrorAction Stop
} catch {
    Refuse ("Hyper-V cannot be used: $($_.Exception.Message) Turn on the Windows feature Microsoft-Hyper-V-All, " +
        "add the user to the group 'Hyper-V Administrators', then sign out and back in (tools\winvm\README.md).")
}
if ($Stop) {
    if (-not $vm) {
        Write-Host "winvm: there is no VM named $name to turn off"
        exit 0
    }
    Stop-VM -Name $name -TurnOff -Force -ErrorAction SilentlyContinue
    $state = (Get-VM -Name $name -ErrorAction SilentlyContinue).State
    if ($state -and $state -ne 'Off') {
        Write-Host "winvm: $name is still $state"
        exit 1
    }
    Write-Host "winvm: $name turned off"
    exit 0
}
if (-not $vm) { Refuse "there is no VM named ${name}: $make." }
if (-not (Get-VMSnapshot -VMName $name -Name ready -ErrorAction SilentlyContinue)) {
    Refuse "the VM $name has no checkpoint 'ready': remove it and $make."
}
if ($vm.State -ne 'Off') {
    Refuse ("the VM $name is $($vm.State): a run never takes over a VM in use. It is left as it is. Turn it off " +
        "when it is free (Stop-VM -Name $name -TurnOff).")
}
$consoles = @(Get-CimInstance Win32_Process -Filter "Name = 'vmconnect.exe'" | Where-Object {
    $_.CommandLine -and ($_.CommandLine -match [regex]::Escape($name) -or $_.CommandLine -match $vm.Id.ToString())
})
if ($consoles) {
    Refuse ("a console window of the VM $name is open (vmconnect.exe): in enhanced session mode it shares the " +
        "host's clipboard. Close it first.")
}
if (-not (Test-Path -LiteralPath $credentialFile)) {
    Refuse "the test account's credential ($credentialFile) is missing: remove the VM and $make."
}
if ($Check) { exit 0 }
$credential = Import-Clixml -Path $credentialFile
$Folder = (Resolve-Path -LiteralPath $Folder).Path

$started = Get-Date
$session = $null

# The run itself, from the restore to the copy out: 0, 1 or 2 as the script's exit. $session is the script's, so
# that the finally below can close it.
function Invoke-Run {
    Restore-VMSnapshot -VMName $name -Name ready -Confirm:$false
    Start-VM -Name $name
    while ((Get-VM -Name $name).Heartbeat -notmatch '^Ok') {
        if ((Seconds $started) -gt $startLimit) { throw "the VM sent no heartbeat within $startLimit s" }
        Start-Sleep -Milliseconds 500
    }
    while (-not $script:session) {
        try {
            $script:session = New-PSSession -VMName $name -Credential $credential -ErrorAction Stop
        } catch {
            if ((Seconds $started) -gt $startLimit) {
                throw "no PowerShell Direct session within $startLimit s: $($_.Exception.Message)"
            }
            Start-Sleep -Seconds 1
        }
    }
    $session = $script:session

    # A restored VM resumes at the checkpoint's time until Hyper-V's time synchronization corrects it (design D3).
    while ($true) {
        $skew = [math]::Abs(((Get-Date).ToUniversalTime() -
            (Invoke-Command -Session $session { (Get-Date).ToUniversalTime() })).TotalSeconds)
        if ($skew -lt 60) { break }
        if ((Seconds $started) -gt $startLimit) { throw "the guest's clock is still $([int]$skew) s off the host's" }
        Start-Sleep -Seconds 1
    }

    $license = Invoke-Command -Session $session -ArgumentList $windowsApplication {
        param($application)
        Get-CimInstance SoftwareLicensingProduct -Filter "PartialProductKey IS NOT NULL AND ApplicationID='$application'" |
            Select-Object -First 1 LicenseStatus, GracePeriodRemaining
    }
    if ($license.LicenseStatus -ne 1 -or $license.GracePeriodRemaining -le 0) {
        Write-Host ("winvm: refused: the VM's evaluation license has run out (status $($license.LicenseStatus), " +
            "$($license.GracePeriodRemaining) minutes left). Make the VM again with tools\winvm\create.ps1 " +
            "(tools\winvm\README.md, 'Making it again after 90 days'), or run the family with run.py --where sandbox.")
        return 2
    }
    if (-not (Invoke-Command -Session $session -ArgumentList $openGL { param($f) Test-Path "$f\opengl32.dll" })) {
        Write-Host ("winvm: refused: the VM has no Mesa OpenGL in $openGL, without which no QB64 program can open " +
            "its window there. It was made before create.ps1 put it there: make the VM again " +
            "(tools\winvm\README.md).")
        return 2
    }
    Write-Host ("winvm: $name ready in {0:N0} s; license: {1:N0} days left" -f (Seconds $started),
        ($license.GracePeriodRemaining / 1440))

    # The run's files in. Folders made by the administrator session are given to tester, who runs the programs.
    Invoke-Command -Session $session -ArgumentList $inside {
        param($root)
        New-Item -ItemType Directory -Force (Join-Path $root 'results') | Out-Null
        & icacls $root /grant 'tester:(OI)(CI)M' /T /Q | Out-Null
    }
    foreach ($item in 'bin', 'list.txt', 'timeout.txt', 'inside.ps1') {
        Copy-Item -ToSession $session -LiteralPath (Join-Path $Folder $item) -Destination "$inside\" -Recurse
    }

    # inside.ps1 in tester's logged-on session, on its desktop: a PowerShell Direct session is in session 0, where a
    # window never appears (design D4, "Why a scheduled task"). Registered fresh on the restored checkpoint.
    Invoke-Command -Session $session -ArgumentList $inside, $taskName, $openGL {
        param($root, $task, $gl)
        $action = New-ScheduledTaskAction -Execute 'powershell.exe' `
            -Argument "-NoProfile -ExecutionPolicy Bypass -File $root\inside.ps1 -OpenGL $gl" -WorkingDirectory $root
        $principal = New-ScheduledTaskPrincipal -UserId 'tester' -LogonType Interactive -RunLevel Limited
        $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries `
            -ExecutionTimeLimit (New-TimeSpan -Hours 1)
        Register-ScheduledTask -TaskName $task -Action $action -Principal $principal -Settings $settings -Force |
            Out-Null
        Start-ScheduledTask -TaskName $task
    }
    $programsStarted = Get-Date

    $done = $false
    while (-not $done -and (Seconds $programsStarted) -lt $Limit) {
        Start-Sleep -Seconds 1
        $done = Invoke-Command -Session $session -ArgumentList $inside { param($root) Test-Path "$root\results\done" }
    }

    $results = Join-Path $Folder 'results'
    New-Item -ItemType Directory -Force $results | Out-Null
    Copy-Item -FromSession $session -Path "$inside\results\*" -Destination $results -Recurse -Force
    if ($done) {
        Write-Host ("winvm: the programs ended after {0:N0} s" -f (Seconds $programsStarted))
        return 0
    }
    Write-Host "winvm: no results\done within $Limit s"
    return 1
}

$code = 1
try {
    # The last value only: a cmdlet's stray output would come before it.
    $code = Invoke-Run | Select-Object -Last 1
} catch {
    Write-Host "winvm: the run failed: $($_.Exception.Message)"
    $code = 1
} finally {
    if ($session) { Remove-PSSession $session -ErrorAction SilentlyContinue }
    Stop-VM -Name $name -TurnOff -Force -ErrorAction SilentlyContinue
    Write-Host ("winvm: $name turned off; {0:N0} s from the restore" -f (Seconds $started))
}
exit $code
