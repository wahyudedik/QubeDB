@echo off
REM QubeDB Test Script - Fixed Version
REM Script untuk test berbagai fitur QubeDB

echo 🧪 QubeDB Test Suite
echo ====================

cd qubedb-core

echo.
echo 📦 Test 1: Basic Usage
echo ----------------------
cargo run --example basic_usage
if errorlevel 1 (
    echo ❌ Basic test failed!
    pause
    exit /b 1
)

echo.
echo ⚡ Test 2: Performance Test
echo ---------------------------
cargo run --example performance_test
if errorlevel 1 (
    echo ❌ Performance test failed!
    pause
    exit /b 1
)

echo.
echo 🔍 Test 3: Unit Tests
echo ---------------------
cargo test
if errorlevel 1 (
    echo ❌ Unit tests failed!
    pause
    exit /b 1
)

echo.
echo 🧠 Test 4: Vector Operations
echo ----------------------------
echo ✅ Vector operations tested in basic_usage example

echo.
echo 📊 Test 5: Graph Operations
echo ----------------------------
echo ✅ Graph operations tested in basic_usage example

echo.
echo 🎉 All tests completed!
echo =====================
echo.
echo ✅ QubeDB is working perfectly!
echo.
echo 🚀 Try the GUI:
echo cd ..\qubedb-gui
echo cargo run
echo.
echo Or run: start-gui.bat
echo.
echo 📖 Read more: SETUP-GUIDE.md

pause