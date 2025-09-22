@echo off
REM QubeDB Dependencies Install Script
REM Script untuk install semua dependencies yang diperlukan

echo 📦 QubeDB Dependencies Installer
echo ================================

echo.
echo 🔍 Checking system requirements...

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust is not installed!
    echo.
    echo 📥 Installing Rust...
    echo Please download and install Rust from: https://rustup.rs/
    echo.
    echo Quick install:
    echo 1. Download: https://win.rustup.rs/x86_64
    echo 2. Run: rustup-init.exe
    echo 3. Restart PowerShell
    echo 4. Run this script again
    pause
    exit /b 1
)

echo ✅ Rust found:
rustc --version

REM Check if Git is installed
git --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Git is not installed!
    echo.
    echo 📥 Installing Git...
    echo Please download and install Git from: https://git-scm.com/download/win
    echo.
    echo Or install via Chocolatey:
    echo choco install git -y
    pause
    exit /b 1
)

echo ✅ Git found:
git --version

REM Check if Node.js is installed
node --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Node.js is not installed!
    echo.
    echo 📥 Installing Node.js...
    echo Please download and install Node.js from: https://nodejs.org/
    echo.
    echo Or install via Chocolatey:
    echo choco install nodejs -y
    pause
    exit /b 1
)

echo ✅ Node.js found:
node --version

echo.
echo 🛠️ Installing Tauri CLI...
cargo install tauri-cli
if errorlevel 1 (
    echo ❌ Failed to install Tauri CLI!
    echo Please check the error messages above
    pause
    exit /b 1
)

echo ✅ Tauri CLI installed successfully!

echo.
echo 🎉 All dependencies installed successfully!
echo ==========================================
echo.
echo ✅ Rust: Ready
echo ✅ Git: Ready
echo ✅ Node.js: Ready
echo ✅ Tauri CLI: Ready
echo.
echo 🚀 Next steps:
echo 1. Build Core: quick-install.bat
echo 2. Build GUI: build-gui.bat
echo 3. Run GUI: start-gui.bat
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
