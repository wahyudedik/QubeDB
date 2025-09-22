@echo off
REM QubeDB Quick Install & Test Script
REM Script otomatis untuk install dan test QubeDB

echo 🚀 QubeDB Quick Install & Test
echo ================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust is not installed!
    echo Please install Rust from https://rustup.rs/
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
    echo Please install Git from https://git-scm.com/download/win
    pause
    exit /b 1
)

echo ✅ Git found:
git --version

echo.
echo 🔨 Building QubeDB Core...
cd qubedb-core

REM Build release version
cargo build --release
if errorlevel 1 (
    echo ❌ Build failed!
    echo Please check the error messages above
    pause
    exit /b 1
)

echo ✅ QubeDB Core built successfully!

echo.
echo 🧪 Testing Basic Functionality...
cargo run --example basic_usage
if errorlevel 1 (
    echo ❌ Basic test failed!
    pause
    exit /b 1
)

echo.
echo ⚡ Testing Performance...
cargo run --example performance_test
if errorlevel 1 (
    echo ❌ Performance test failed!
    pause
    exit /b 1
)

echo.
echo 🎉 All tests completed successfully!
echo ====================================
echo.
echo ✅ QubeDB is working correctly!
echo.
echo 🚀 Next steps:
echo 1. Try the GUI: cd ..\qubedb-gui && cargo run
echo 2. Read documentation: SETUP-GUIDE.md
echo 3. Test with your projects
echo.
echo 📖 Documentation: https://github.com/wahyudedik/QubeDB
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
