@echo off
REM QubeDB Complete Test Suite
REM Script untuk test semua fitur QubeDB

echo 🧪 QubeDB Complete Test Suite
echo =============================

echo.
echo 📦 Test 1: Core Functionality
echo ------------------------------
cd qubedb-core
cargo run --example basic_usage
if errorlevel 1 (
    echo ❌ Core test failed!
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
echo 🖥️ Test 4: GUI Build Test
echo -------------------------
cd ..\qubedb-gui
cargo build --release
if errorlevel 1 (
    echo ❌ GUI build failed!
    pause
    exit /b 1
)

echo ✅ GUI build successful!

echo.
echo 🎉 All tests completed successfully!
echo ====================================
echo.
echo ✅ Core Functionality: Working
echo ✅ Performance: Excellent
echo ✅ Unit Tests: Passing
echo ✅ GUI Build: Successful
echo.
echo 🚀 QubeDB is ready for use!
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
