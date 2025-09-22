@echo off
REM Tauri CLI Install Script
REM Script untuk install Tauri CLI

echo 🛠️ Installing Tauri CLI
echo =======================

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust is not installed!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo ✅ Rust found:
rustc --version

echo.
echo 📦 Installing Tauri CLI...
cargo install tauri-cli
if errorlevel 1 (
    echo ❌ Failed to install Tauri CLI!
    echo Please check the error messages above
    pause
    exit /b 1
)

echo ✅ Tauri CLI installed successfully!

echo.
echo 🎉 Tauri CLI installation completed!
echo ====================================
echo.
echo ✅ Tauri CLI: Ready
echo.
echo 🚀 Next steps:
echo 1. Build GUI: build-gui.bat
echo 2. Run GUI: start-gui.bat
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
