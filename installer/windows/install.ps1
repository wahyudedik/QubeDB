# QubeDB Windows Installer Script
# PowerShell script untuk install QubeDB di Windows

param(
    [string]$InstallPath = "$env:ProgramFiles\QubeDB",
    [switch]$Force,
    [switch]$Silent
)

# Colors untuk output
$Red = "`e[31m"
$Green = "`e[32m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

function Write-ColorOutput {
    param([string]$Message, [string]$Color = $Reset)
    Write-Host "$Color$Message$Reset"
}

function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-Rust {
    Write-ColorOutput "🦀 Installing Rust..." $Blue
    
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        Write-ColorOutput "✅ Rust already installed: $(rustc --version)" $Green
        return
    }
    
    # Download dan install Rust
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        Write-ColorOutput "📥 Downloading Rust installer..." $Yellow
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
        
        Write-ColorOutput "🔧 Installing Rust..." $Yellow
        & $rustupPath -y
        
        # Add Rust to PATH
        $env:PATH += ";$env:USERPROFILE\.cargo\bin"
        [Environment]::SetEnvironmentVariable("PATH", $env:PATH, [EnvironmentVariableTarget]::User)
        
        Write-ColorOutput "✅ Rust installed successfully!" $Green
    }
    catch {
        Write-ColorOutput "❌ Failed to install Rust: $($_.Exception.Message)" $Red
        throw
    }
    finally {
        if (Test-Path $rustupPath) {
            Remove-Item $rustupPath -Force
        }
    }
}

function Install-QubeDB {
    Write-ColorOutput "📦 Installing QubeDB..." $Blue
    
    # Create installation directory
    if (-not (Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }
    
    # Clone repository
    $repoPath = "$env:TEMP\QubeDB"
    if (Test-Path $repoPath) {
        Remove-Item $repoPath -Recurse -Force
    }
    
    try {
        Write-ColorOutput "📥 Cloning QubeDB repository..." $Yellow
        git clone https://github.com/qubedb/qubedb.git $repoPath
        
        # Build QubeDB Core
        Write-ColorOutput "🔨 Building QubeDB Core..." $Yellow
        Set-Location "$repoPath\qubedb-core"
        cargo build --release
        
        # Build QubeDB GUI
        Write-ColorOutput "🖥️ Building QubeDB GUI..." $Yellow
        Set-Location "$repoPath\qubedb-gui"
        cargo build --release
        
        # Copy binaries
        Write-ColorOutput "📋 Installing binaries..." $Yellow
        Copy-Item "$repoPath\qubedb-core\target\release\qubedb-core.exe" "$InstallPath\qubedb.exe"
        Copy-Item "$repoPath\qubedb-gui\target\release\qubedb-gui.exe" "$InstallPath\qubedb-gui.exe"
        
        # Copy GUI assets
        Copy-Item "$repoPath\qubedb-gui\dist" "$InstallPath\gui" -Recurse -Force
        
        Write-ColorOutput "✅ QubeDB installed successfully!" $Green
    }
    catch {
        Write-ColorOutput "❌ Failed to install QubeDB: $($_.Exception.Message)" $Red
        throw
    }
    finally {
        Set-Location $PSScriptRoot
        if (Test-Path $repoPath) {
            Remove-Item $repoPath -Recurse -Force
        }
    }
}

function Install-Service {
    Write-ColorOutput "🔧 Installing QubeDB Service..." $Blue
    
    $serviceName = "QubeDB"
    $serviceDisplayName = "QubeDB Database Service"
    $serviceDescription = "QubeDB Multi-Model Database Service"
    $servicePath = "$InstallPath\qubedb.exe"
    
    # Check if service already exists
    $existingService = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if ($existingService) {
        if ($Force) {
            Write-ColorOutput "🔄 Removing existing service..." $Yellow
            Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
            sc.exe delete $serviceName
        } else {
            Write-ColorOutput "⚠️ Service already exists. Use -Force to replace." $Yellow
            return
        }
    }
    
    try {
        # Create service
        Write-ColorOutput "📋 Creating Windows service..." $Yellow
        New-Service -Name $serviceName -BinaryPathName $servicePath -DisplayName $serviceDisplayName -Description $serviceDescription -StartupType Automatic
        
        Write-ColorOutput "✅ QubeDB service installed successfully!" $Green
    }
    catch {
        Write-ColorOutput "❌ Failed to install service: $($_.Exception.Message)" $Red
        throw
    }
}

function Create-DesktopShortcut {
    Write-ColorOutput "🖥️ Creating desktop shortcuts..." $Blue
    
    $desktopPath = [Environment]::GetFolderPath("Desktop")
    $startMenuPath = [Environment]::GetFolderPath("StartMenu")
    
    # QubeDB GUI shortcut
    $guiShortcut = "$desktopPath\QubeDB Desktop.lnk"
    $guiWshShell = New-Object -ComObject WScript.Shell
    $guiShortcutObj = $guiWshShell.CreateShortcut($guiShortcut)
    $guiShortcutObj.TargetPath = "$InstallPath\qubedb-gui.exe"
    $guiShortcutObj.WorkingDirectory = $InstallPath
    $guiShortcutObj.Description = "QubeDB Desktop GUI"
    $guiShortcutObj.Save()
    
    # Start Menu shortcuts
    $startMenuFolder = "$startMenuPath\Programs\QubeDB"
    if (-not (Test-Path $startMenuFolder)) {
        New-Item -ItemType Directory -Path $startMenuFolder -Force | Out-Null
    }
    
    $startMenuShortcut = "$startMenuFolder\QubeDB Desktop.lnk"
    $startMenuShortcutObj = $guiWshShell.CreateShortcut($startMenuShortcut)
    $startMenuShortcutObj.TargetPath = "$InstallPath\qubedb-gui.exe"
    $startMenuShortcutObj.WorkingDirectory = $InstallPath
    $startMenuShortcutObj.Description = "QubeDB Desktop GUI"
    $startMenuShortcutObj.Save()
    
    Write-ColorOutput "✅ Desktop shortcuts created!" $Green
}

function Set-EnvironmentVariables {
    Write-ColorOutput "🌍 Setting environment variables..." $Blue
    
    # Add QubeDB to PATH
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", [EnvironmentVariableTarget]::Machine)
    if ($currentPath -notlike "*$InstallPath*") {
        $newPath = "$currentPath;$InstallPath"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, [EnvironmentVariableTarget]::Machine)
    }
    
    # Set QUBEDB_HOME
    [Environment]::SetEnvironmentVariable("QUBEDB_HOME", $InstallPath, [EnvironmentVariableTarget]::Machine)
    
    Write-ColorOutput "✅ Environment variables set!" $Green
}

function Show-InstallationComplete {
    Write-ColorOutput "`n🎉 QubeDB Installation Complete!" $Green
    Write-ColorOutput "=================================" $Green
    Write-ColorOutput "📁 Installation Path: $InstallPath" $Blue
    Write-ColorOutput "🖥️ Desktop Shortcut: QubeDB Desktop" $Blue
    Write-ColorOutput "🔧 Service: QubeDB (Automatic startup)" $Blue
    Write-ColorOutput "`n🚀 Quick Start:" $Yellow
    Write-ColorOutput "1. Double-click 'QubeDB Desktop' on your desktop" $Blue
    Write-ColorOutput "2. Or run: qubedb-gui.exe" $Blue
    Write-ColorOutput "3. Or run: qubedb.exe --help" $Blue
    Write-ColorOutput "`n📖 Documentation: https://docs.qubedb.com" $Blue
    Write-ColorOutput "🐛 Issues: https://github.com/qubedb/qubedb/issues" $Blue
}

# Main installation process
function Main {
    Write-ColorOutput "🚀 QubeDB Windows Installer" $Green
    Write-ColorOutput "============================" $Green
    Write-ColorOutput "Installing QubeDB to: $InstallPath" $Blue
    
    # Check if running as administrator
    if (-not (Test-Administrator)) {
        Write-ColorOutput "❌ This script requires administrator privileges!" $Red
        Write-ColorOutput "Please run PowerShell as Administrator and try again." $Yellow
        exit 1
    }
    
    # Check if installation already exists
    if ((Test-Path $InstallPath) -and (-not $Force)) {
        Write-ColorOutput "⚠️ QubeDB already installed at: $InstallPath" $Yellow
        $response = Read-Host "Do you want to reinstall? (y/N)"
        if ($response -ne 'y' -and $response -ne 'Y') {
            Write-ColorOutput "Installation cancelled." $Yellow
            exit 0
        }
    }
    
    try {
        # Installation steps
        Install-Rust
        Install-QubeDB
        Install-Service
        Create-DesktopShortcut
        Set-EnvironmentVariables
        Show-InstallationComplete
        
        Write-ColorOutput "`n✅ Installation completed successfully!" $Green
    }
    catch {
        Write-ColorOutput "`n❌ Installation failed: $($_.Exception.Message)" $Red
        Write-ColorOutput "Please check the error messages above and try again." $Yellow
        exit 1
    }
}

# Run main function
Main
