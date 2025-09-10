$ErrorActionPreference = 'Stop'

# Config
$downloadUrl = "https://github.com/kevinj045/rew/releases/latest/download/rew.exe"
$rewRoot = "$env:LOCALAPPDATA\rew"
$installDir = Join-Path $rewRoot "bin"
$tempFile = "$env:TEMP\rew.exe"

Write-Host "⬇️  Downloading rew.exe..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile

# Ensure directory structure
$folders = @("apps", "data", "bin", "config")
foreach ($f in $folders) {
    $path = Join-Path $rewRoot $f
    if (-not (Test-Path $path)) {
        Write-Host "📂 Creating $path"
        New-Item -ItemType Directory -Force -Path $path | Out-Null
    }
}

# Move binary to bin
Write-Host "📁 Installing to: $installDir"
Copy-Item $tempFile -Destination (Join-Path $installDir "rew.exe") -Force

# Check if vcredist is installed
function Test-VCRedistInstalled {
    $keys = @(
        "HKLM:\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64",
        "HKLM:\SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64",
        "HKLM:\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x86",
        "HKLM:\SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x86"
    )

    foreach ($key in $keys) {
        if (Test-Path $key) {
            $installed = (Get-ItemProperty $key -ErrorAction SilentlyContinue).Installed
            if ($installed -eq 1) { return $true }
        }
    }
    return $false
}

if (-not (Test-VCRedistInstalled)) {
    Write-Host "📦 Installing Visual C++ Redistributable..."
    $vcUrl = "https://aka.ms/vs/17/release/vc_redist.x64.exe"
    $vcInstaller = "$env:TEMP\vc_redist.x64.exe"
    Invoke-WebRequest -Uri $vcUrl -OutFile $vcInstaller
    Start-Process $vcInstaller -ArgumentList "/quiet", "/norestart" -Wait
} else {
    Write-Host "ℹ️  Visual C++ Redistributable already installed"
}

# Add bin folder to PATH if not present
$pathType = "User"
$envPath = [Environment]::GetEnvironmentVariable("Path", $pathType)
if ($envPath -notlike "*$installDir*") {
    Write-Host "🔧 Adding $installDir to $pathType PATH"
    [Environment]::SetEnvironmentVariable("Path", "$envPath;$installDir", $pathType)
} else {
    Write-Host "ℹ️  $installDir already in PATH"
}

Write-Host "✅ Installation complete. Restart your terminal to use 'rew'."
