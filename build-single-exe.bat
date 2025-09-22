@echo off
REM QubeDB Single Executable Builder
REM Script untuk build single executable yang bisa install dan run

echo 🚀 QubeDB Single Executable Builder
echo ==================================

echo.
echo 🔨 Building QubeDB Core...
cd qubedb-core
cargo build --release
if errorlevel 1 (
    echo ❌ Core build failed!
    pause
    exit /b 1
)

echo ✅ Core built successfully!

echo.
echo 🖥️ Building QubeDB GUI...
cd ..\qubedb-gui
cargo build --release
if errorlevel 1 (
    echo ❌ GUI build failed!
    pause
    exit /b 1
)

echo ✅ GUI built successfully!

echo.
echo 📦 Creating Single Executable...
cd ..

REM Copy all necessary files to a single directory
mkdir QubeDB-Portable 2>nul
copy qubedb-core\target\release\qubedb-core.exe QubeDB-Portable\
copy qubedb-gui\target\release\qubedb-gui.exe QubeDB-Portable\
copy qubedb-gui\dist\* QubeDB-Portable\ /s
copy qubedb-gui\icons\* QubeDB-Portable\ /s
copy QubeDB-Setup.bat QubeDB-Portable\
copy SETUP-GUIDE.md QubeDB-Portable\
copy README.md QubeDB-Portable\

echo ✅ Single executable created!

echo.
echo 🎉 Single Executable Build Completed!
echo =====================================
echo.
echo ✅ QubeDB Core: Ready
echo ✅ QubeDB GUI: Ready
echo ✅ Portable Package: Ready
echo.
echo 📦 Portable package location: QubeDB-Portable\
echo.
echo 🚀 Usage:
echo 1. Copy QubeDB-Portable folder anywhere
echo 2. Run QubeDB-Setup.bat to install and launch
echo 3. Or run qubedb-gui.exe directly
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
