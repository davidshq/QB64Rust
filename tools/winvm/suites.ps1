# The suites VM's part of one `check` run (tools\winvm\README.md, "The suites VM"; OpenSpec change vm-suites,
# design D3 to D6). tools\check\check.py calls it:
#
#   tools\winvm\suites.ps1 -Check               the refusals that need no VM started; the VM is not touched
#   tools\winvm\suites.ps1 -Start -Tree <path>  restore the checkpoint "ready", start the VM, check its clock and
#                                               license, attach the cache disk at the tree's parent folder, and
#                                               bring the copies of the QB64pe clone and of the tree up to date
#   tools\winvm\suites.ps1 -Stop                take the cache disk offline in the guest, turn the VM off, and
#                                               detach the cache disk
#
# Between -Start and -Stop, tools\winvm\step.ps1 runs the run's steps in the VM, each in a session of its own.
#
# The guest sees the tree at the host's own path: the cache disk's volume is mounted at the tree's parent folder,
# so that <parent>\<tree> and <parent>\QB64pe are on it, and any path built into an executable made on the host is
# right in the guest (design D5). The cache disk also holds qb64rust-vm\: Mesa, and the stamps of what was copied.
#
# Exit 0: done. Exit 2: refused, with a line "suites-vm: refused: <why>" saying what to do; the VM is left as it was
# found. Exit 1: -Start failed after the VM was started; check.py then calls -Stop.

param(
    [Parameter(ParameterSetName = 'Check', Mandatory = $true)][switch]$Check,
    [Parameter(ParameterSetName = 'Start', Mandatory = $true)][switch]$Start,
    [Parameter(ParameterSetName = 'Start', Mandatory = $true)][string]$Tree,
    [Parameter(ParameterSetName = 'Stop', Mandatory = $true)][switch]$Stop
)

$ErrorActionPreference = 'Stop'
$name = if ($env:QB64RUST_SUITES_VM) { $env:QB64RUST_SUITES_VM } else { 'qb64rust-suites' }
$credentialFile = Join-Path $env:LOCALAPPDATA "qb64rust\winvm\$name\credential.xml"
$own = 'qb64rust-vm'
$windowsApplication = '55c92734-d682-4d71-983e-d6ec3f16059f'
$startLimit = 120

function Refuse([string]$message) {
    Write-Host "suites-vm: refused: $message"
    exit 2
}

function Seconds([datetime]$since) { ((Get-Date) - $since).TotalSeconds }

$make = 'make it with tools\winvm\create.ps1 -Shape Suites (tools\winvm\README.md, "The suites VM"), or run ' +
    'check with --where host'
try {
    Import-Module Hyper-V -ErrorAction Stop
    $vmHost = Get-VMHost -ErrorAction Stop
    $vm = Get-VM -Name $name -ErrorAction SilentlyContinue
} catch {
    Refuse ("Hyper-V cannot be used: $($_.Exception.Message) Turn on the Windows feature Microsoft-Hyper-V-All and " +
        "join the group 'Hyper-V Administrators' (tools\winvm\README.md).")
}
$cacheDisk = Join-Path $vmHost.VirtualHardDiskPath "$name\cache.vhdx"

# The cache disk's drive, if the VM's settings hold it: a drive added to a running VM stays there after the VM is
# turned off and after a restore (design.md, "Measured", task 1.2).
function Remove-CacheDrive {
    Get-VMHardDiskDrive -VMName $name | Where-Object { $_.Path -eq $cacheDisk } | Remove-VMHardDiskDrive
}

if ($Stop) {
    if (-not $vm) {
        Write-Host "suites-vm: there is no VM named $name to turn off"
        exit 0
    }
    if ((Get-VM -Name $name).State -eq 'Running' -and (Test-Path -LiteralPath $credentialFile)) {
        # Writes out what the guest still holds of the cache disk: turning off is a power cut (design D4).
        try {
            $credential = Import-Clixml -Path $credentialFile
            $session = New-PSSession -VMName $name -Credential $credential -ErrorAction Stop
            try {
                Invoke-Command -Session $session {
                    Get-Disk | Where-Object { -not $_.IsBoot -and -not $_.IsSystem -and -not $_.IsOffline } |
                        Set-Disk -IsOffline $true
                }
            } finally {
                Remove-PSSession $session
            }
        } catch {
            Write-Host "suites-vm: the cache disk could not be taken offline first: $($_.Exception.Message)"
        }
    }
    Stop-VM -Name $name -TurnOff -Force -ErrorAction SilentlyContinue
    $state = (Get-VM -Name $name).State
    if ($state -ne 'Off') {
        Write-Host "suites-vm: $name is still $state"
        exit 1
    }
    Remove-CacheDrive
    Write-Host "suites-vm: $name turned off"
    exit 0
}

# The refusals that leave the VM untouched (design D6).
if (-not $vm) { Refuse "there is no VM named ${name}: $make." }
if (-not (Get-VMSnapshot -VMName $name -Name ready -ErrorAction SilentlyContinue)) {
    Refuse "the VM $name has no checkpoint 'ready': remove it and $make."
}
if ($vm.State -ne 'Off') {
    Refuse ("the VM $name is $($vm.State): a run never takes over a VM in use, and it is left as it is. When it is " +
        "free, turn it off with tools\winvm\suites.ps1 -Stop.")
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
if (-not (Test-Path -LiteralPath $cacheDisk)) {
    Refuse "the VM $name has no cache disk ($cacheDisk): remove it and $make."
}
if ($Check) { exit 0 }

$Tree = (Resolve-Path -LiteralPath $Tree).Path.TrimEnd('\')
$parent = Split-Path $Tree
$treeName = Split-Path $Tree -Leaf
$clone = Join-Path $parent 'QB64pe'
if (-not $parent -or $parent.Length -le 3) {
    Refuse "the tree ($Tree) is at the root of a drive: the cache disk is mounted at the tree's parent folder."
}
if (-not (Test-Path -LiteralPath (Join-Path $clone 'internal\c'))) {
    Refuse "there is no QB64pe clone beside the tree ($clone), which the steps read."
}
$credential = Import-Clixml -Path $credentialFile
# Scratch files of the copies, on the host: in the tree's target\, which git ignores.
$scratch = Join-Path $Tree 'target\check\vm'
New-Item -ItemType Directory -Force $scratch | Out-Null

$started = Get-Date
$session = $null

function Invoke-Start {
    Remove-CacheDrive
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

    # A restored VM resumes at the checkpoint's time until Hyper-V's time synchronization corrects it.
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
        Write-Host ("suites-vm: refused: the VM's evaluation license has run out (status $($license.LicenseStatus), " +
            "$($license.GracePeriodRemaining) minutes left). Make it again from the base (tools\winvm\README.md, " +
            "'The suites VM'), or run check with --where host.")
        return 2
    }

    # The cache disk, at the tree's parent folder (design D4, D5). A disk taken offline by the last -Stop comes back
    # offline: it is put online here.
    Add-VMHardDiskDrive -VMName $name -ControllerType SCSI -Path $cacheDisk
    $mounted = Invoke-Command -Session $session -ArgumentList $parent {
        param($parent)
        $deadline = (Get-Date).AddSeconds(30)
        $disk = $null
        while (-not $disk -and (Get-Date) -lt $deadline) {
            Update-HostStorageCache
            $disk = Get-Disk | Where-Object { -not $_.IsBoot -and -not $_.IsSystem } | Select-Object -First 1
            if (-not $disk) { Start-Sleep -Milliseconds 300 }
        }
        if (-not $disk) { return 'the guest did not see the cache disk within 30 s' }
        if ($disk.IsOffline) { Set-Disk -Number $disk.Number -IsOffline $false }
        if ($disk.IsReadOnly) { Set-Disk -Number $disk.Number -IsReadOnly $false }
        $partition = Get-Partition -DiskNumber $disk.Number | Where-Object { $_.Type -eq 'Basic' } | Select-Object -First 1
        if (-not $partition) { return 'the cache disk has no data partition' }
        $drive = $parent.Substring(0, 2)
        if ($drive -ieq $env:SystemDrive) {
            New-Item -ItemType Directory -Force $parent | Out-Null
            Add-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -AccessPath "$parent\"
        } else {
            # A tree on another drive of the host: the cache disk takes that drive's letter, the parent is a folder on it.
            Set-Partition -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -NewDriveLetter $drive[0]
            New-Item -ItemType Directory -Force $parent | Out-Null
        }
        ''
    }
    if ($mounted) { throw "the cache disk could not be mounted: $mounted" }
    Write-Host ("suites-vm: $name ready in {0:N0} s; license: {1:N0} days left" -f (Seconds $started),
        ($license.GracePeriodRemaining / 1440))

    # The clone, without .git, when its commit or uncommitted changes differ from the copy's (design D3).
    $head = (& git -C $clone rev-parse HEAD) -join ''
    $changes = (& git -C $clone status --porcelain) -join "`n"
    $sha = [Security.Cryptography.SHA256]::Create()
    $stamp = $head + ' ' + ([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($changes))) -replace '-', '')
    $guestStamp = Invoke-Command -Session $session -ArgumentList "$parent\$own\clone.stamp" {
        param($p) if (Test-Path -LiteralPath $p) { (Get-Content -Raw -LiteralPath $p).Trim() } else { '' }
    }
    if ($guestStamp -ne $stamp) {
        $t = Get-Date
        $archive = Join-Path $scratch 'clone.tar'
        & "$env:SystemRoot\System32\tar.exe" -cf $archive --exclude ./.git -C $clone .
        if ($LASTEXITCODE -ne 0) { throw "tar could not archive the clone (exit $LASTEXITCODE)" }
        Copy-Item -ToSession $session -LiteralPath $archive -Destination "$parent\$own\clone.tar"
        Remove-Item -LiteralPath $archive
        $code = Invoke-Command -Session $session -ArgumentList $parent, $own, $stamp {
            param($parent, $own, $stamp)
            $clone = "$parent\QB64pe"
            if (Test-Path -LiteralPath $clone) { Remove-Item -Recurse -Force -LiteralPath $clone }
            New-Item -ItemType Directory -Force $clone | Out-Null
            & "$env:SystemRoot\System32\tar.exe" -xf "$parent\$own\clone.tar" -C $clone
            $code = $LASTEXITCODE
            Remove-Item -LiteralPath "$parent\$own\clone.tar"
            if ($code -eq 0) { Set-Content -LiteralPath "$parent\$own\clone.stamp" -Value $stamp }
            $code
        }
        if ($code -ne 0) { throw "tar could not unpack the clone in the guest (exit $code)" }
        Write-Host ("suites-vm: the clone copied in ({0:N0} s)" -f (Seconds $t))
    }

    # The tree: the files git tracks or would track, as the Linux sync takes them. The guest removes the files of
    # the last list that are not in this one, and nothing else, so its target\ stays (design D3).
    $t = Get-Date
    $files = @(& git -C $Tree ls-files -co --exclude-standard) | Where-Object { [IO.File]::Exists((Join-Path $Tree $_)) }
    $list = Join-Path $scratch 'tree.list'
    [IO.File]::WriteAllText($list, (($files -join "`n") + "`n"), (New-Object Text.UTF8Encoding($false)))
    $archive = Join-Path $scratch 'tree.tar'
    & "$env:SystemRoot\System32\tar.exe" -cf $archive -C $Tree -T $list
    if ($LASTEXITCODE -ne 0) { throw "tar could not archive the tree (exit $LASTEXITCODE)" }
    Invoke-Command -Session $session -ArgumentList "$parent\$treeName" {
        param($copy) New-Item -ItemType Directory -Force $copy | Out-Null
    }
    Copy-Item -ToSession $session -LiteralPath $archive -Destination "$parent\$own\$treeName.tar"
    Copy-Item -ToSession $session -LiteralPath $list -Destination "$parent\$own\$treeName.new.list"
    Remove-Item -LiteralPath $archive
    $result = Invoke-Command -Session $session -ArgumentList $parent, $own, $treeName {
        param($parent, $own, $treeName)
        $copy = "$parent\$treeName"
        $old = "$parent\$own\$treeName.list"
        $new = "$parent\$own\$treeName.new.list"
        $removed = 0
        if (Test-Path -LiteralPath $old) {
            $keep = New-Object 'System.Collections.Generic.HashSet[string]'
            foreach ($f in [IO.File]::ReadAllLines($new)) { [void]$keep.Add($f) }
            foreach ($f in [IO.File]::ReadAllLines($old)) {
                if ($f -and -not $keep.Contains($f)) {
                    $p = Join-Path $copy $f
                    if ([IO.File]::Exists($p)) { [IO.File]::Delete($p); $removed++ }
                }
            }
        }
        & "$env:SystemRoot\System32\tar.exe" -xf "$parent\$own\$treeName.tar" -C $copy
        $code = $LASTEXITCODE
        Remove-Item -LiteralPath "$parent\$own\$treeName.tar"
        if ($code -eq 0) { Move-Item -Force -LiteralPath $new -Destination $old }
        "$code $removed"
    }
    $code, $removed = $result -split ' '
    if ($code -ne '0') { throw "tar could not unpack the tree in the guest (exit $code)" }
    Write-Host ("suites-vm: the tree copied in: {0} files, {1} removed ({2:N0} s)" -f $files.Count, $removed, (Seconds $t))
    return 0
}

$code = 1
try {
    # The last value only: a cmdlet's stray output would come before it.
    $code = Invoke-Start | Select-Object -Last 1
} catch {
    Write-Host "suites-vm: -Start failed: $($_.Exception.Message)"
    $code = 1
} finally {
    if ($session) { Remove-PSSession $session -ErrorAction SilentlyContinue }
}
# On a failure the VM is left running: check.py turns it off with -Stop, and its last line above is the reason
# check.py reports.
exit $code
