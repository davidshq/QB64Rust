# What the suites VM gets that create.ps1 and game.ps1 both need, pinned by SHA-256 (tools\winvm\README.md, "The
# suites VM"). Read with Import-PowerShellDataFile.
#
# Node.js runs a game port's browser tests (tools\winvm\game.ps1). The VM has no Node: game.ps1 copies this zip onto
# the cache disk until a VM made by create.ps1 -Shape Suites has it installed. Checked 2026-11-17 against
# https://nodejs.org/dist/v24.22.3/SHASUMS256.txt (v24.22.3: LTS "Krypton", a security release of 2026-11-10).
@{
    Node = @{
        Version = '24.22.3'
        File    = 'node-v24.22.3-win-x64.zip'
        Url     = 'https://nodejs.org/dist/v24.22.3/node-v24.22.3-win-x64.zip'
        Sha256  = 'e612af99f626ab415ea225af6bce5c971feb448cb5cae250bb1c3fa50c5f7d9f'
    }
}
