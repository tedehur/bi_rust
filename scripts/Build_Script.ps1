[CmdletBinding()]
param(
    [string]$ProjectPath = (Split-Path -Parent $PSScriptRoot),
    [switch]$Release
)

$ErrorActionPreference = 'Stop'

function Invoke-Cargo {
    param([string[]]$CargoArgs)
    Write-Host ("cargo " + ($CargoArgs -join ' ')) -ForegroundColor Cyan
    & cargo @CargoArgs
    if ($LASTEXITCODE -ne 0) {
        throw "cargo a échoué avec le code $LASTEXITCODE"
    }
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Cargo est introuvable. Installez Rust stable avec rustup, puis rouvrez le terminal.'
}

$root = (Resolve-Path -LiteralPath $ProjectPath).Path
if (-not (Test-Path -LiteralPath (Join-Path $root 'Cargo.toml'))) {
    throw "Cargo.toml introuvable dans $root"
}

Push-Location $root
try {
    Invoke-Cargo -CargoArgs @('test')
    if ($Release) {
        Invoke-Cargo -CargoArgs @('build', '--release')
    } else {
        Invoke-Cargo -CargoArgs @('build')
    }
    Write-Host 'Tests et compilation Rust terminés. Aucun flash matériel n’a été effectué.' -ForegroundColor Green
}
finally {
    Pop-Location
}
