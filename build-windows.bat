@echo off
REM QubeDB Windows Build Script
REM Builds QubeDB Core, GUI, and creates Windows installer

setlocal enabledelayedexpansion

echo 🚀 QubeDB Windows Build Script
echo ================================

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

REM Check if Git is installed
git --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Git is not installed!
    echo Please install Git from https://git-scm.com/
    pause
    exit /b 1
)

echo ✅ Git found:
git --version

REM Create build directory
if not exist "build" mkdir build
if not exist "build\windows" mkdir build\windows
if not exist "build\windows\installer" mkdir build\windows\installer

echo.
echo 🔨 Building QubeDB Core...
cd qubedb-core
cargo build --release
if errorlevel 1 (
    echo ❌ Failed to build QubeDB Core!
    pause
    exit /b 1
)
echo ✅ QubeDB Core built successfully!

echo.
echo 🖥️ Building QubeDB GUI...
cd ..\qubedb-gui
cargo build --release
if errorlevel 1 (
    echo ❌ Failed to build QubeDB GUI!
    pause
    exit /b 1
)
echo ✅ QubeDB GUI built successfully!

echo.
echo 📦 Preparing installer files...
cd ..\build\windows

REM Copy binaries
copy "..\..\qubedb-core\target\release\qubedb-core.exe" "qubedb.exe"
copy "..\..\qubedb-gui\target\release\qubedb-gui.exe" "qubedb-gui.exe"

REM Copy GUI assets
if not exist "gui" mkdir gui
xcopy "..\..\qubedb-gui\dist\*" "gui\" /E /I /Y

REM Copy documentation
copy "..\..\README.md" "README.txt"
copy "..\..\LICENSE" "LICENSE.txt"
echo # QubeDB Changelog > CHANGELOG.txt
echo Version 1.0.0 - Initial release >> CHANGELOG.txt
echo - Core database engine >> CHANGELOG.txt
echo - Desktop GUI application >> CHANGELOG.txt
echo - Windows installer >> CHANGELOG.txt

REM Copy installer script
copy "..\..\installer\windows\qubedb-installer.nsi" "installer\"

echo.
echo 🔧 Creating Windows installer...

REM Check if NSIS is installed
makensis /VERSION >nul 2>&1
if errorlevel 1 (
    echo ❌ NSIS is not installed!
    echo Please install NSIS from https://nsis.sourceforge.io/
    echo.
    echo Alternative: Use PowerShell installer
    echo Running PowerShell installer instead...
    cd ..\..\installer\windows
    powershell -ExecutionPolicy Bypass -File install.ps1
    pause
    exit /b 0
)

echo ✅ NSIS found:
makensis /VERSION

REM Create installer
cd installer
makensis qubedb-installer.nsi
if errorlevel 1 (
    echo ❌ Failed to create installer!
    pause
    exit /b 1
)

echo ✅ Windows installer created successfully!

REM Move installer to build directory
move "QubeDB-Setup.exe" "..\..\QubeDB-Setup.exe"

echo.
echo 🎉 Build completed successfully!
echo ================================
echo 📁 Build directory: build\windows
echo 📦 Installer: QubeDB-Setup.exe
echo.
echo 🚀 To install QubeDB:
echo 1. Run QubeDB-Setup.exe
echo 2. Follow the installation wizard
echo 3. Launch QubeDB Desktop from desktop shortcut
echo.
echo 📖 Documentation: https://docs.qubedb.com
echo 🐛 Issues: https://github.com/qubedb/qubedb/issues

pause
